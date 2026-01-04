mod commands;
use commands::command::*;
use commands::monitoring;
use tokio;
use clap::{Parser,Subcommand};
use anyhow::{Ok, Result};


#[derive(Parser)]
#[command(name = "kali")]
#[command(version = "0.1.0")]
#[command(about = "cli for easier\nuse as tools and easier\n\"made by dex0o0\"\tgit hub:\"https://github.com/dex0o0\"")]
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
    Monitoring,
}
#[derive(Subcommand)]
enum WifiAction{
    List,
    Connect{
        #[arg(value_name = "NETWORK_NAME")]
        name:String
    },
    Connection,
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
                WifiAction::Connection => {connection();},
            }
        },
        Commands::Monitoring => {monitoring::add_path_log();},
    }

    Ok(())
}