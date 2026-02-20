use std::result::Result::Ok;
use core::fmt;
use std::process::Command;
use anyhow::Result;
use std::path::PathBuf;
use crate::commands::scan_sys;

// pub fn download(chat:String){
//     let new = Link::new();
//     let link = Link::extract(&chat);
    

// }

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
    let out= Command::new("nmcli")
    .arg("device")
    .arg("wifi")
    .arg("connect")
    .arg(name)
    .output()
    .expect("Error from connect to wifi");
    let status = String::from(format!("{:?}",out.status));
    if status == "exit status 0"{
        println!("connected");
    }else {
        println!("failed connecting");
    }
}
pub fn scan_status()-> Result<()>{
    let mut sysinfo = scan_sys::Sysinfo::new();
    sysinfo.auto_fill().expect("Error auto fill data");
    sysinfo.display();
    Ok(())
}
fn current_dir()-> PathBuf{
    let current_dir = env::current_exe()
    .expect("Error from get path binery file")
    .parent().expect("Error get parent path")
    .parent().expect("Error get parent path")
    .parent().expect("Error get parent path")
    .to_path_buf();
    current_dir
}
fn ch_adress(path:&'static str)->String{
    let c_d=current_dir();
    let mut  file_path = String::new();
    file_path = format!("{}/{}",c_d.display(),path);
    file_path
}
pub async fn open_git(){
    let current_dir = current_dir();
    let mut script = String::new();
    script = format!("{}/src/scripts/git.sh",current_dir.display());
    // println!("{}",&script);
    let _ = Command::new("sh")
    .arg(script)
    .output().expect("Error from run script open git");
}

pub async fn open_gmail()->Result<()>{
    let _ = Command::new("google-chrome-stable")
    .args(["--app=https://accounts.google.com/b/0/AddMailService"," --start-fullscreen ","--new-window"])
    .output();
    Ok(())
}

pub async fn open_youtube_music()-> Result<()>{
    let _ = Command::new("google-chrome-stable")
    .args(["--app=https://music.youtube.com/"," --start-fullscreen"," --new-window"])
    .output();
    Ok(())
}
pub fn notif_send(title:String,body:String,time:String){
    let script = ch_adress("src/scripts/notif.sh");
    let _ = Command::new("sh")
    .arg(script)
    .arg(title)
    .arg(body)
    .arg(time)
    .output()
    .expect("Error from run notif.sh");
}

pub async fn github()-> Result<()>{
    let _ = Command::new("google-chrome-stable")
    .args( ["--app=https://github.com/"," --start-fullscreen", "--new-window"])
    .output();
    Ok(())
}
pub async fn chrome()->Result<()>{
    let _ = Command::new("google-chrome-stable").output();
    Ok(())
}