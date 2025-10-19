use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use win11_act_types::{ActivationError, Config, Result};

#[cfg(windows)]
pub fn is_elevated() -> bool {
    use windows::Win32::Foundation::BOOL;
    use windows::Win32::System::SystemServices::SE_GROUP_ENABLED;
    use windows::core::PWSTR;

    unsafe {
        let mut token = std::mem::zeroed();
        if windows::Win32::System::Threading::OpenProcessToken(
            windows::Win32::System::Threading::GetCurrentProcess(),
            windows::Win32::Security::TOKEN_QUERY,
            &mut token,
        ).is_err() {
            return false;
        }

        let mut elevation = std::mem::zeroed();
        let mut size = 0;
        if windows::Win32::Security::GetTokenInformation(
            token,
            windows::Win32::Security::TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of_val(&elevation) as u32,
            &mut size,
        ).is_err() {
            let _ = windows::Win32::Foundation::CloseHandle(token);
            return false;
        }

        let _ = windows::Win32::Foundation::CloseHandle(token);
        elevation != BOOL(0)
    }
}

#[cfg(not(windows))]
pub fn is_elevated() -> bool {
    false
}

pub fn install_license_files(config: &Config) -> Result<()> {
    let target_dir = Path::new(r"C:\Windows\System32\spp\tokens\skus");

    if config.verbose {
        println!("Installing license files to: {}", target_dir.display());
    }

    let files = win11_act_embed::get_license_files();

    for file in files {
        let target_path = target_dir.join(&file.relative_path);

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ActivationError::DirectoryCreation(format!("{}: {}", parent.display(), e))
            })?;
        }

        if config.dry_run {
            if config.verbose {
                println!("[DRY RUN] Would write: {}", target_path.display());
            }
        } else {
            fs::write(&target_path, file.data).map_err(|e| {
                ActivationError::FileWrite(format!("{}: {}", target_path.display(), e))
            })?;

            if config.verbose {
                println!("Wrote: {}", target_path.display());
            }
        }
    }

    Ok(())
}

fn run_slmgr_command(args: &[&str], config: &Config) -> Result<()> {
    let windir = std::env::var("windir").unwrap_or_else(|_| r"C:\Windows".to_string());
    let script_path = format!(r"{}\system32\slmgr.vbs", windir);

    if config.verbose {
        println!("Running: cscript.exe {} {}", script_path, args.join(" "));
    }

    if config.dry_run {
        if config.verbose {
            println!("[DRY RUN] Would execute slmgr command");
        }
        return Ok(());
    }

    let mut cmd = Command::new("cscript.exe");
    cmd.arg(&script_path);

    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ActivationError::CommandExecution(format!("Failed to execute: {}", e)))?;

    if config.verbose {
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    if !output.status.success() {
        return Err(ActivationError::CommandExecution(format!(
            "Command failed with status: {}",
            output.status
        )));
    }

    Ok(())
}

pub fn activate_windows(config: &Config) -> Result<()> {
    if config.verbose {
        println!("Starting Windows 11 LTSC activation process...");
    }

    run_slmgr_command(&["/rilc"], config)?;

    let _ = run_slmgr_command(&["/upk"], config);
    let _ = run_slmgr_command(&["/ckms"], config);
    let _ = run_slmgr_command(&["/cpky"], config);

    run_slmgr_command(&["/ipk", &config.product_key], config)?;
    run_slmgr_command(&["/skms", &config.kms_server], config)?;
    run_slmgr_command(&["/ato"], config)?;

    if config.verbose {
        println!("Activation completed successfully!");
    }

    Ok(())
}

pub fn run_activation(config: &Config) -> Result<()> {
    if !is_elevated() {
        return Err(ActivationError::InsufficientPrivileges);
    }

    install_license_files(config)?;
    activate_windows(config)?;

    Ok(())
}
