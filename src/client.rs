/**
 * the client of scrcpy
 */
use super::util::{Device, ResHelper, ResourceName};

pub struct ScrcpyClient {
    pub device: Device,
}

impl ScrcpyClient {
    pub fn new(device: Device) -> ScrcpyClient {
        Self { device }
    }

    pub fn push_server_file(&self) {
        let res = self.device.cmd_push(
            ResHelper::get_file_path(ResourceName::ScrcpyServer).unwrap(),
            "/data/local/tmp/scrcpy-server.jar",
        );

        match res {
            Ok(info) => {
                println!("Successfully push server files.\n{}", info)
            }
            Err(e) => {
                eprintln!("Failed to push server files\n{}", e)
            }
        }
    }
}
