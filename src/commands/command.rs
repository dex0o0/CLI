use anyhow::Result;
use std::process::{Command, Stdio};
use std::result::Result::Ok;

pub fn list_network() {
    let out = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("Error from run command  \'--show\'");

    if out.status.success() {
        let result = String::from_utf8_lossy(&out.stdout);
        println!("{}", result);
    }
}
pub fn connection() {
    match Command::new("nmcli").arg("connection").output() {
        Ok(o) => println!("{}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => eprintln!("Error:{}", e),
    }
}
pub fn disconnect(wifi_name: String) {
    let out = Command::new("nmcli")
        .arg("connection")
        .output()
        .expect("Error from get output \"nmcli connectin\"");

    if out.status.success() {
        match Command::new("nmcli")
            .arg("connection")
            .arg("down")
            .arg(&wifi_name)
            .output()
        {
            Ok(val) => println!("{}", String::from_utf8_lossy(&val.stdout)),
            Err(e) => eprintln!("Error:{}", e),
        }
    }
}
pub fn connet_to_wifi(name: String) {
    let out = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("connect")
        .arg(name)
        .output()
        .expect("Error from connect to wifi");
    let status = String::from(format!("{:?}", out.status));
    if status == "exit status 0" {
        println!("connected");
    } else {
        println!("failed connecting");
    }
}

// #[warn(dead_code)]
// pub fn scan_status()-> Result<()>{
//     let mut sysinfo = scan_sys::Sysinfo::new();
//     sysinfo.auto_fill().expect("Error auto fill data");
//     sysinfo.display();
//     Ok(())
// }

pub async fn open_gmail() -> Result<()> {
    let _ = Command::new("google-chrome-stable")
        .args([
            "--app=https://accounts.google.com/b/0/AddMailService",
            "--new-window",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

pub async fn open_youtube_music() -> Result<()> {
    let _ = Command::new("google-chrome-stable")
        .args([
            "--app=https://music.youtube.com/",
            "--start-fullscreen",
            "--new-window",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}
pub fn notif_send(_title: String, _body: String, _time: String) {
    todo!()
}

pub async fn github() -> Result<()> {
    let _ = Command::new("google-chrome-stable")
        .args([
            "--app=https://github.com/",
            "--start-fullscreen",
            "--new-window",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

#[allow(dead_code)]
pub async fn chrome() -> Result<()> {
    let _ = Command::new("google-chrome-stable").spawn()?;
    Ok(())
}
