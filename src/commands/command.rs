use std::process::Command;
use anyhow::Result;
use super::scan_sys;
pub fn list_network(){
    let out = Command::new("nmcli")
    .arg("device")
    .arg("wifi")
    .arg("list")
    .output()
    .expect("Error from run command  \'--show\'");

    if out.status.success(){
        let result = String::from_utf8_lossy(&out.stdout);
        println!("{}",result);
    }
}
pub fn disconnect(device_name:String){
    let out = Command::new("nmcli")
    .arg("connection")
    .output()
    .expect("Error from get output \"nmcli connectin\"");
    
    if out.status.success(){
        let output_str = String::from_utf8_lossy(&out.stdout);
        for line in output_str.lines(){
            let columns:Vec<&str> = line.split_whitespace().collect();
            if columns.len() >= 4{
                let device = columns[3];
                if device_name == device {
                    let name_connection=columns[0];
                    let out_disconnect = Command::new("nmcli")
                    .arg("connection")
                    .arg("down")
                    .arg(&name_connection)
                    .output()
                    .expect("Error from disconnecting..");
                    println!("{}",String::from_utf8_lossy(&out_disconnect.stdout));
                }
            }
        }
    }
}
pub fn connet_to_wifi(name:String){
    let out = Command::new("nmcli")
    .arg("device")
    .arg("wifi")
    .arg("connect")
    .arg(name)
    .output()
    .expect("Error from connect to wifi");
    println!("{:?}",out);
}
pub fn scan_status()-> Result<()>{
    let mut sysinfo = scan_sys::Sysinfo::new();
    sysinfo.auto_fill().expect("Error auto fill data");
    sysinfo.display();
    Ok(())
}