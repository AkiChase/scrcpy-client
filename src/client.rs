use std::{io::BufRead, thread};

/**
 * the client of scrcpy
 */
use super::util::{Device, ResHelper, ResourceName};

#[derive(Debug)]
pub struct ScrcpyClient {
    pub device: Device,
    pub version: String,
}

impl ScrcpyClient {
    pub fn new(device: Device) -> ScrcpyClient {
        let file_name = ResHelper::get_file_path(ResourceName::ScrcpyServer);
        let version = &file_name[file_name.rfind('v').unwrap() + 1..];
        Self {
            device,
            version: version.to_string(),
        }
    }

    /// push server file to current device
    pub fn push_server_file(&self) {
        let res = self.device.cmd_push(
            ResHelper::get_file_path(ResourceName::ScrcpyServer),
            "/data/local/tmp/scrcpy-server.jar",
        );

        match res {
            Ok(info) => {
                println!("Successfully push server files.\n{}", info);
            }
            Err(e) => {
                eprintln!("Failed to push server files.\n{}", e);
            }
        }
    }

    /// forward port to current device
    pub fn forward_server_port(&self) {
        if let Err(e) = self.device.cmd_forward("tcp:27183", "localabstract:scrcpy") {
            eprintln!("Failed to forward port.\n{}", e);
        } else {
            println!("Successfully forward port.");
        }
    }

    /// push server file to current device
    pub fn shell_start_server(&self) -> thread::JoinHandle<()> {
        let res = self.device.cmd_shell(&[
            "CLASSPATH=/data/local/tmp/scrcpy-server.jar",
            "app_process",
            "/",
            "com.genymobile.scrcpy.Server",
            &self.version,
        ]);
        
        thread::spawn(|| {
            match res {
                Err(e) => {
                    eprintln!("{}", e);
                }
                Ok(mut child) => {
                    let out = child.stdout.take().unwrap();
                    let mut out = std::io::BufReader::new(out);
                    let mut s = String::new();

                    while let Ok(_) = out.read_line(&mut s) {
                        // break at the end of program
                        if let Ok(Some(_)) = child.try_wait() {
                            break;
                        }
                        print!("{}", s);
                        // clear string to store new line only
                        s.clear();
                    }
                }
            }
        })
    }
}
