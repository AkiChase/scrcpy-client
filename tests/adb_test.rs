use std::io::BufRead;

use scrcpy_client::util::{Adb, ResHelper, ResourceName};

#[test]
fn test_cmd_devices() {
    let devices = Adb::cmd_devices().unwrap();
    for device in &devices {
        println!("device: {}|{}", device.id, device.status)
    }
}

#[test]
fn test_cmd_kill_server() {
    Adb::cmd_kill_server().unwrap();
}

#[test]
fn test_cmd_start_server() {
    Adb::cmd_start_server().unwrap()
}

#[test]
fn test_cmd_reverse_remove() {
    Adb::cmd_reverse_remove().unwrap()
}

#[test]
fn test_cmd_push() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        let res = devices[0].cmd_push(
            ResHelper::get_file_path(ResourceName::ScrcpyServer),
            "/data/local/tmp/scrcpy-server.jar",
        );

        println!("{}", res.unwrap());
    }
}

#[test]
fn test_cmd_reverse() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        devices[0]
            .cmd_reverse("localabstract:scrcpy", "tcp:27183")
            .unwrap();
    }
}

#[test]
fn test_cmd_forward() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        devices[0]
            .cmd_forward("tcp:27183", "localabstract:scrcpy")
            .unwrap();
    }
}

#[test]
fn test_cmd_shell() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        let res = devices[0].cmd_shell(&["ping", "-c", "3", "127.0.0.1"]);
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
    }
}
