use scrcpy_client::client::ScrcpyClient;
use scrcpy_client::util::Adb;

#[test]
fn test_push_server_file() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        let client: ScrcpyClient = ScrcpyClient::new(devices[0].clone());
        client.push_server_file();
    }
}