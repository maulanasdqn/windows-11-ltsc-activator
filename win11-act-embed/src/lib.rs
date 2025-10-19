use std::path::PathBuf;
use win11_act_types::LicenseFile;

pub fn get_license_files() -> Vec<LicenseFile> {
    vec![
        LicenseFile {
            name: "csvlk-pack-ppdlic.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-ppdlic.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-ppdlic.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-10-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-10-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-10-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-10-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-10-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-10-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-10-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-1-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-1-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-1-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-1-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-1-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-1-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-2-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-2-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-2-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-2-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-2-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-2-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-2-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-3-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-3-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-3-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-3-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-3-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-3-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-3-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-4-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-4-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-4-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-4-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-4-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-4-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-4-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-5-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-5-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-5-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-5-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-5-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-5-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-5-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-6-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-6-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-6-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-6-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-6-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-6-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-6-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-7-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-7-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-7-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-7-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-7-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-7-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-7-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-8-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-8-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-8-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-8-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-8-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-8-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-8-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-9-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-9-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-9-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-9-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-9-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "csvlk-pack-Volume-CSVLK-9-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/csvlk-pack-Volume-CSVLK-9-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "DefaultPpd-csvlk-pack-ppdlic.xrm-ms".to_string(),
            data: include_bytes!("../assets/csvlk-pack/DefaultPpd-csvlk-pack-ppdlic.xrm-ms"),
            relative_path: PathBuf::from("csvlk-pack/DefaultPpd-csvlk-pack-ppdlic.xrm-ms"),
        },
        LicenseFile {
            name: "DefaultPpd-EnterpriseS-ppdlic.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/DefaultPpd-EnterpriseS-ppdlic.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/DefaultPpd-EnterpriseS-ppdlic.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-DM-1-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-DM-1-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-DM-1-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-DM-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-DM-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-DM-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-DM-1-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-DM-1-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-DM-1-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-DM-1-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-DM-1-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-DM-1-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-NONSLP-1-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-NONSLP-1-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-NONSLP-1-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-NONSLP-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-NONSLP-1-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-OEM-NONSLP-1-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-OEM-NONSLP-1-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-ppdlic.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-ppdlic.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-ppdlic.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-1-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-1-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-1-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-1-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-1-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-1-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-2-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-2-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-2-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-2-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-2-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-2-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-2-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-3-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-3-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-3-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-3-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-3-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-3-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-3-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-4-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-4-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-4-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-4-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-4-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-4-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-4-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-5-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-5-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-5-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-5-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-5-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-5-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-5-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-6-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-6-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-6-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-6-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-6-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-CSVLK-6-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-CSVLK-6-ul-store-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-GVLK-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-GVLK-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-GVLK-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-GVLK-1-ul-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-GVLK-1-ul-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-GVLK-1-ul-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-MAK-1-pl-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-MAK-1-pl-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-MAK-1-pl-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-MAK-1-ul-oob-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-MAK-1-ul-oob-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-MAK-1-ul-oob-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-MAK-1-ul-phn-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-MAK-1-ul-phn-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-MAK-1-ul-phn-rtm.xrm-ms"),
        },
        LicenseFile {
            name: "EnterpriseS-Volume-MAK-1-ul-store-rtm.xrm-ms".to_string(),
            data: include_bytes!("../assets/EnterpriseS/EnterpriseS-Volume-MAK-1-ul-store-rtm.xrm-ms"),
            relative_path: PathBuf::from("EnterpriseS/EnterpriseS-Volume-MAK-1-ul-store-rtm.xrm-ms"),
        },
    ]
}
