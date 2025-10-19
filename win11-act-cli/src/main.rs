use clap::Parser;
use win11_act_core::run_activation;
use win11_act_types::Config;

#[derive(Parser)]
#[command(name = "Windows 11 LTSC Activator")]
#[command(version = "0.1.0")]
#[command(about = "Activates Windows 11 Enterprise LTSC", long_about = None)]
struct Cli {
    #[arg(short = 's', long, default_value = "kms.digiboy.ir")]
    kms_server: String,

    #[arg(short = 'k', long, default_value = "M7XTQ-FN8P6-TTKYV-9D4CC-J462D")]
    product_key: String,

    #[arg(short = 'd', long)]
    dry_run: bool,

    #[arg(short = 'v', long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config {
        kms_server: cli.kms_server,
        product_key: cli.product_key,
        dry_run: cli.dry_run,
        verbose: cli.verbose,
    };

    if config.dry_run {
        println!("=== DRY RUN MODE ===");
        println!("KMS Server: {}", config.kms_server);
        println!("Product Key: {}", config.product_key);
        println!();
    }

    match run_activation(&config) {
        Ok(_) => {
            if !config.dry_run {
                println!("Windows 11 LTSC activated successfully!");
            } else {
                println!("Dry run completed successfully!");
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Activation failed: {}", e);
            std::process::exit(1);
        }
    }
}
