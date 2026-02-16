mod commands{
    pub mod dl;
    pub mod extractLinks;
    pub mod monitoring;
    pub mod command;
    pub mod scan_sys;
}
use commands::command::*;
use tokio;
use clap::{Parser,Subcommand,CommandFactory};
use clap_complete::{Generator, Shell, generate};
use anyhow::{Ok, Result};


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
        #[arg(value_name = "link download",short='u',long="url")]
        url:String,
        #[arg(short='o',long="filename")]
        name:Option<String>,
    },
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
        Commands::Status => {scan_status().expect("Error from scan your system");},
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
        Commands::Dl { url,name } => {
            match name {
                Some(name)=>{
                    if let Err(e) = commands::dl::download_with_filename(&url, &name).await{
                        eprintln!("Error:{}",e);
                    }
                }
                None=>{
                    if let Err(e) = commands::dl::download(&url).await{
                        eprintln!("Error:{}",e);
                    }
                }
            }
            
        },
        Commands::Complation { shell } =>{
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }
    Ok(())
}