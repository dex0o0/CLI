mod commands;
use std::{fmt::Display, result};
use commands::scan_sys;
use serde::de;
use tokio;
use clap::{Parser,Subcommand, builder::Str};
use anyhow::{Ok, Result};
use env::current_dir;
use std::process::Command;


#[derive(Parser)]
#[command(name = "kali")]
#[command(version = "0.1.0")]
#[command(about = "cli for easyli")]
struct  Cli{
    #[command(subcommand)]
    commad:Commands,


}


#[derive(Subcommand)]
enum Commands {
    Status,
    Wifi{
       #[command(subcommand)]
       action:WifiAction,
    },
}
#[derive(Subcommand)]
enum WifiAction{
    List,
    Connect{
        #[arg(value_name = "NETWORK_NAME")]
        name:String
    },
    Disconnect{
        #[arg(value_name = "NETWORK_DEVICE", default_value = "wlan0")]
        device_name:String,
    },
}
#[tokio::main]
async fn main()-> Result<()>{
    let cli = Cli::parse();

    match cli.commad {
        Commands::Status => {scan_status().expect("Error from scan your system");},
        Commands::Wifi { action } =>{
           match action {
               WifiAction::List => {list_network();},
               WifiAction::Connect { name } => {connet_to_wifi(name);},
                WifiAction::Disconnect { device_name } => {disconnect(device_name);},
           }
        },
    }

    Ok(())
}

fn list_network(){
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
fn disconnect(device_name:String){
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
fn connet_to_wifi(name:String){
    let out = Command::new("nmcli")
    .arg("device")
    .arg("wifi")
    .arg("connect")
    .arg(name)
    .output()
    .expect("Error from connect to wifi");
    println!("{:?}",out);
}
fn scan_status()-> Result<()>{
    let mut sysinfo = scan_sys::Sysinfo::new();
    sysinfo.auto_fill().expect("Error auto fill data");
    sysinfo.display();
    Ok(())
}