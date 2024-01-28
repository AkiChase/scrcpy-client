use scrcpy_client::util::Adb;

#[test]
fn test_cmd_devices() {
    let devices = Adb::cmd_devices();
    match devices {
        Ok(list)=>{
            for device in &list{
                println!("{}|{}", device.id, device.status)
            }
        },
        Err(e)=>println!("{}", e)
    }
}

