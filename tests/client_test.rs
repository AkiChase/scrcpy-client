use scrcpy_client::client::ScrcpyClient;
use scrcpy_client::util::Adb;

#[test]
fn test_push_server_file() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone());
        println!("{:?}", client);
        client.push_server_file();
    }
}

#[test]
fn test_reverse_server_port() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone());
        client.reverse_server_port();
    }
}

#[test]
fn test_shell_start_server() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone());
        let h = client.shell_start_server();
        h.join().unwrap();
    }
}

#[test]
fn test_fullly_start_server() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!");
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone());
        client.push_server_file();
        client.reverse_server_port();
        // todo: open socket
        let h = client.shell_start_server();
        h.join().unwrap();
    }
}