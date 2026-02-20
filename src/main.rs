mod commands{
    pub mod dl;
    pub mod extractLinks;
    pub mod monitoring;
    pub mod command;
    pub mod scan_sys;
    pub mod tui;
    pub mod config;
}
use std::{path::PathBuf, str::FromStr};
use colored::{self, Colorize};
use commands::command::*;
use tokio;
use clap::{Parser,Subcommand,CommandFactory,Args};
use clap_complete::{ Shell, generate};
use anyhow::{ Result, anyhow};
use crate::commands::{config::conf::{self, Birthday, Email, Month, save_and_report}, dl::{dl_read_file, download, download_with_filename}, tui::TuiApp};


#[derive(Parser)]
#[command(name = "dex")]
#[command(version = "0.1.034")]
#[command(about = "CLI for easier\nuse as tools and easier\n\"made by dex0o0\"\tgit hub:\"https://github.com/dex0o0\"")]
struct  Cli{
    #[command(subcommand)]
    commad:Commands,


}


#[derive(Subcommand)]
pub enum Commands {
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
    Config(ConfArg),
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

#[derive(Args)]
pub struct ConfArg {

    #[arg(short='G',long="global")]
    pub global:bool,

    pub key:String,
    pub value:String,
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
        Commands::Status => {let _ = TuiApp::show_status();},
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
            let _= chrome().await;
            let _= github().await;
            let _=open_gmail().await;
            let _=open_youtube_music().await;
        },
        Commands::Notif{title,body,time} => {
            notif_send(title,body,time);
        },
        Commands::Dl { url,name,file } => {
           let _ = if let Some(file_path) = file {
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
        },
        Commands::Config(args)=>{
            let mut current_user = conf::load_config(args.global).unwrap_or_else(|_| conf::User::empty());
            if args.key == "user.email"{
                match Email::new(args.value.clone()) {
                    Ok(email)=>{
                        current_user.email = Some(email);
                        save_and_report(&current_user, args.global);
                    },
                    Err(e)=>eprintln!("[{}]:{}","ERROR".red(),e),
                }
            }else if args.key == "user.name" {
                current_user.name = Some(args.value.clone());
                save_and_report(&current_user, args.global);
            }else if args.key == "user.birthday" || args.key == "user.birth" {
                let parts:Vec<&str>=args.value.split('-').collect();
                if parts.len() != 3{
                    eprintln!("Please use format:YYY-Month-DD (e.g., 2000-January-15");
                }else {
                    let year = parts[0].parse::<u16>().unwrap_or(0);
                    let month = Month::from_str(parts[1]);
                    let day = parts[2].parse::<u8>().unwrap_or(0);
                    match month{
                        Ok(m)=>{
                            match Birthday::new(year,m,day) {
                                Ok(bday)=>{
                                    current_user.birthday = Some(bday);
                                    save_and_report(&current_user, args.global);
                                }
                                Err(e)=>eprintln!("[{}]:{}","ERROR".red(),e),
                            }
                        }
                        Err(e)=>eprintln!("[{}]:{}","ERROR".red(),e),
                    }
                }
            
            }else {
                println!("Key '{}' is not supported",args.key.yellow());
            }

        }
    }
    Ok(())
}