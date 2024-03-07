use anyhow::{anyhow, Ok, Result};
use std::path::Path;

pub enum ResourceName {
    Adb,
    ScrcpyServer,
}

pub struct ResHelper;

impl ResHelper {
    pub fn res_init() -> Result<()> {
        for p in [Path::new("res"), Path::new("res/adb")] {
            if !p.exists() {
                return Err(anyhow!(format!(
                    "Resource missing! {}",
                    p.to_str().unwrap()
                )));
            }
        }
        Ok(())
    }
    pub fn get_file_path(file_name: ResourceName) -> &'static str {
        match file_name {
            #[cfg(target_os = "windows")]
            ResourceName::Adb => "res/adb.exe",
            #[cfg(not(target_os = "windows"))]
            ResourceName::Adb => "res/adb",

            ResourceName::ScrcpyServer => "res/scrcpy-server-v2.3.1",
        }
    }
}
