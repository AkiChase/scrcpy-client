use crate::util::AppError;

use super::ResHelper;
use std::{io::BufRead, process::Command};

pub struct Device {
    pub id: String,
    pub status: String,
}

pub struct Adb;
impl Adb {
    fn cmd_base() -> Command {
        Command::new(ResHelper::get_file_path("adb").unwrap())
    }

    /// execute "adb devices" and return devices list or error
    pub fn cmd_devices() -> Result<Vec<Device>, AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command.args(&["devices"]).output();

        match res {
            Ok(output) => {
                let mut devices_vec: Vec<Device> = Vec::new();
                let mut lines = output.stdout.lines();
                // skip the first line
                lines.next();
                // parse string to Device
                for line in lines {
                    if let Ok(s) = line {
                        let device_info: Vec<&str> = s.split('\t').collect();
                        if device_info.len() == 2 {
                            devices_vec.push(Device {
                                id: device_info[0].to_string(),
                                status: device_info[1].to_string(),
                            })
                        }
                    }
                }
                return Ok(devices_vec);
            }
            Err(e) => {
                return Err(AppError {
                    type_name: "Adb".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }

    pub fn cmd_push(device_id: &str, src: &str, des: &str) -> Result<String, AppError> {
        let mut adb_command = Adb::cmd_base();
        let res = adb_command
            .args(&["-s", device_id, "push", src, des])
            .output();

        match res {
            Ok(output) => {
                return Ok(String::from_utf8(output.stdout).unwrap());
            }
            Err(e) => {
                return Err(AppError {
                    type_name: "Adb".to_string(),
                    message: e.to_string(),
                })
            }
        }
    }
}
