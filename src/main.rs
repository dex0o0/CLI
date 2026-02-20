mod commands{
    pub mod dl;
    pub mod extractLinks;
    pub mod monitoring;
    pub mod command;
    pub mod scan_sys;
    pub mod tui;
}
use std::{any, path::{Path, PathBuf}};
use colored::{self, Colorize};
use commands::command::*;
use tokio;
use clap::{Parser,Subcommand,CommandFactory};
use clap_complete::{Generator, Shell, generate};
use anyhow::{Ok, Result, anyhow};

use crate::commands::{dl::{dl_read_file, download, download_with_filename}, tui::TuiApp};


#[derive(Parser)]
#[command(name = "dex")]
#[command(version = "0.1.034")]
#[command(about = "CLI for easier\nuse as tools and easier\n\"made by dex0o0\"\tgit hub:\"https://github.com/dex0o0\"")]
struct  Cli{
    #[command(subcommand)]
    commad:Commands,


}


#[derive(Subcommand)]
enum Commands {
    #[command(name="status",
    about="show system status",
    long_about="show data hardware your system\n
    cpu,gpu,tempreture of gpu,memory,memory use and...",
    )]
    Status,

    #[command(name="git",about="open main page github")]
    Git,

    #[command(name="gmail",about="open gmail page")]
    Gmail,

    #[command(name="ym",about="open youtube music")]
    YM,

    #[command(name="wifi",about="wifi manager use by pakege 'nmcli'")]
    Wifi{
       #[command(subcommand)]
       action:WifiAction,
    },

    #[command(name="monitoring",about="switch to monitoring mod")]
    Monitoring,

    #[command(name="codemod",about="switch to coding mod",long_about="open git,gmail and youtube music")]
    Codemod,

    #[command(name="notif",about="set notif for any time")]
    Notif{
        title:String,
        body:String,
        time:String,
    },
    
    #[command(name="dl",about="download a link")]
    Dl{
        #[arg(value_name = "link download",short='u',long="url",help="| link download")]
        url:Option<String>,
        #[arg(short='o',long="filename",help="| set name for file downloaded")]
        name:Option<String>,

        #[arg(short='f',long="file",help="| read link in file")]
        file:Option<PathBuf>,
    },
    TODO,
    Complation{
        shell:Shell,
    },
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
        Commands::Status => {TuiApp::show_status();},
        Commands::Wifi { action } =>{
           match action {
               WifiAction::List => {list_network();},
               WifiAction::Connect { name } => {connet_to_wifi(name);},
                WifiAction::Disconnect { device_name } => {disconnect(device_name);},
                WifiAction::Connection => {connection();},
            }
        },
        Commands::Monitoring => {todo!()},
        Commands::Git => {open_git().await},
        Commands::YM => {open_youtube_music().await.expect("failed open youtube music")},
        Commands::Gmail => {open_gmail().await.expect("Error to open gmail")},
        Commands::Codemod => {

            let _= github().await;
            let _=open_gmail().await;
            let _=open_youtube_music().await;
        },
        Commands::Notif{title,body,time} => {
            notif_send(title,body,time);
        },
        Commands::Dl { url,name,file } => {
           let result = if let Some(file_path) = file {
                if !file_path.exists(){
                    return Err(anyhow!("[{}]file dose not exists","ERROR".red()));
                }
                dl_read_file(file_path).await.expect("Error");
           }else if let (Some(url),Some(name)) = (&url,name) {
                download_with_filename(&url, &name).await.expect("Error");
           }else if let Some(u) = &url {
                download(&u).await.expect("Error");
           }else {
               eprintln!("option not found please dex dl --help");
               return Ok(());
           };
        },
        Commands::TODO => {},
        Commands::Complation { shell } =>{
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }
    Ok(())
}