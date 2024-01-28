use scrcpy_client::util::Adb;
use scrcpy_client::util::ResHelper;

#[test]
fn test_cmd_devices() {
    let devices = Adb::cmd_devices().unwrap();
    for device in &devices {
        println!("{}|{}", device.id, device.status)
    }
}

#[test]
fn test_cmd_push() {
    let devices = Adb::cmd_devices().unwrap();
    if devices.len() < 1 {
        println!("no devices!")
    } else {
        let res = Adb::cmd_push(
            &devices[0].id,
            ResHelper::get_file_path("scrcpy-server").unwrap(),
            "/data/local/tmp/scrcpy-server.jar",
        );

        println!("{}", res.unwrap());
    }
}
