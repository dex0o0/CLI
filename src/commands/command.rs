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
pub fn connection(){
    match Command::new("nmcli")
    .arg("connection")
    .output() {
        Ok(o) => println!("{}",String::from_utf8_lossy(&o.stdout)),
        Err(e)=> eprintln!("Error:{}",e)
    }
}
pub fn disconnect(wifi_name:String){
    let out = Command::new("nmcli")
    .arg("connection")
    .output()
    .expect("Error from get output \"nmcli connectin\"");
    
    if out.status.success(){
        match Command::new("nmcli")
        .arg("connection")
        .arg("down")
        .arg(&wifi_name)
        .output() {
            Ok(val)=> println!("{}",String::from_utf8_lossy(&val.stdout)) ,
            Err(e)=> eprintln!("Error:{}",e), 
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