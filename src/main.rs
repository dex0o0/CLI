mod commands {
    pub mod command;
    pub mod config;
    pub mod dl;
    pub mod extractlinks;
    pub mod mod_screen;
    pub mod module;
    pub mod monitoring;
    pub mod scan_sys;
    pub mod tui;
}
use crate::commands::{
    config::conf::{self, save_and_report, Birthday, Email, Month},
    dl::{dl_read_file, download, download_with_filename},
    mod_screen::gamemod,
    module::{deepsearch::search, readstory, smalmodule},
    monitoring,
    tui::TuiApp,
};
use anyhow::{anyhow, Result};
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::{self, Colorize};
use commands::command::*;
use std::{env::current_dir, path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(name = "dex")]
#[command(version = "0.1.421")]
#[command(
    about = "CLI for easier works\n\n\"made by dex0o0\"\tgit hub:\"https://github.com/dex0o0\""
)]
struct Cli {
    #[command(subcommand)]
    commad: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        name = "status",
        about = "show system status",
        long_about = "show data hardware your system\n
    cpu,gpu,tempreture of gpu,memory,memory use and..."
    )]
    Status,

    #[command(name = "git", about = "open main page github")]
    Git,

    #[command(name = "gmail", about = "open gmail page")]
    Gmail,

    #[command(name = "ym", about = "open youtube music")]
    YM,

    #[command(name = "wifi", about = "wifi manager use by pakege 'nmcli'")]
    Wifi {
        #[command(subcommand)]
        action: WifiAction,
    },
    #[command(name = "config", about = "config user and save data")]
    Config(ConfArg),

    #[command(name = "monitoring", about = "switch to monitoring mod")]
    Monitoring,

    #[command(about = "show list disk")]
    Disk,

    #[command(
        name = "codemod",
        about = "switch to coding mod",
        long_about = "open git,gmail and youtube music"
    )]
    Codemod,

    #[command(about = "on/off game mod")]
    Gamemod,

    #[command(name = "notif", about = "set notif for any time")]
    Notif {
        title: String,
        body: String,
        time: String,
    },

    #[command(name = "dl", about = "download a link")]
    Dl {
        #[arg(
            value_name = "link download",
            short = 'u',
            long = "url",
            help = "| link download"
        )]
        url: Option<String>,

        #[arg(
            short = 'o',
            long = "filename",
            help = "| set name for file downloaded"
        )]
        name: Option<String>,

        #[arg(short = 'f', long = "file", help = "| read link in file")]
        file: Option<PathBuf>,
    },

    #[command(name = "ls", about = "deep sreach in system")]
    Search {
        #[arg(short = 'p', long = "path", help = "path directory for search")]
        path: Option<PathBuf>,

        #[arg(short = 't', long = "target", help = "target for deepsearch")]
        target: Option<String>,

        #[arg(short = 'd', long = "depth", help = "depth search in path")]
        depth: Option<usize>,

        #[arg(
            short = 'b',
            long = "block",
            help = "set lot of directories for don't searched"
        )]
        block: Option<String>,
    },

    #[command(about = "generate auto suggestions")]
    Complation { shell: Shell },

    #[command(
        name = "rds",
        about = "read file and type data on screen.\n
    [how to use]\n\tyou can run this command for read storyes\n\t\"dex rds <file-path>\"\n
    [how to write story]\n\tyou can use lot of command for print colorize and waite as some point\n\n\n
    [commands]\n\t\"\\[r,g,b,y,B]\\<note>\\\"\tthis is structuer for print colorize [red,green,blue,yellow,and {B} for bold text]
    \n\t\"_\"\t\t\tthis charecter can help you for set delay in printing
    \n\t \t\t\tyou can place this charecter wherever in your story"
    )]
    ReadStory {
        #[arg(help = "file path")]
        path_file: PathBuf,
    },
}

#[derive(Args)]
pub struct ConfArg {
    #[arg(short = 'G', long = "global")]
    pub global: bool,

    pub key: String,
    pub value: String,
}

#[derive(Subcommand)]
pub enum WifiAction {
    List,
    Connect {
        #[arg(value_name = "NETWORK_NAME")]
        name: String,
    },

    Connection,
    Disconnect {
        #[arg(value_name = "NETWORK_DEVICE", default_value = "wlan0")]
        device_name: String,
    },
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.commad {
        Commands::Status => {
            let _ = TuiApp::show_status();
        }
        Commands::Wifi { action } => match action {
            WifiAction::List => {
                list_network();
            }
            WifiAction::Connect { name } => {
                connet_to_wifi(name);
            }
            WifiAction::Disconnect { device_name } => {
                disconnect(device_name);
            }
            WifiAction::Connection => {
                connection();
            }
        },
        Commands::Monitoring => {
            todo!()
        }
        Commands::Git => open_git().await,
        Commands::YM => open_youtube_music()
            .await
            .expect("failed open youtube music"),
        Commands::Gmail => open_gmail().await.expect("Error to open gmail"),
        Commands::Codemod => {
            // let _= chrome().await;
            let _ = github().await;
            let _ = open_gmail().await;
            let _ = open_youtube_music().await;
        }
        Commands::Notif { title, body, time } => {
            notif_send(title, body, time);
        }
        Commands::Dl { url, name, file } => {
            if let Some(file_path) = file {
                if !file_path.exists() {
                    return Err(anyhow!("[{}]file dose not exists", "ERROR".red()));
                }
                dl_read_file(file_path).await.expect("Error");
            } else if let (Some(url), Some(name)) = (&url, name) {
                download_with_filename(url, &name).await.expect("Error");
            } else if let Some(u) = &url {
                download(u).await.expect("Error");
            } else {
                eprintln!("option not found please dex dl --help");
                return Ok(());
            };
        }
        Commands::Config(args) => {
            let mut current_user =
                conf::load_config(args.global).unwrap_or_else(|_| conf::User::empty());
            if args.key == "user.email" {
                match Email::new(args.value.clone()) {
                    Ok(email) => {
                        current_user.email = Some(email);
                        save_and_report(&current_user, args.global);
                    }
                    Err(e) => eprintln!("[{}]:{}", "ERROR".red(), e),
                }
            } else if args.key == "user.name" {
                current_user.name = Some(args.value.clone());
                save_and_report(&current_user, args.global);
            } else if args.key == "user.birthday" || args.key == "user.birth" {
                let parts: Vec<&str> = args.value.split('-').collect();
                if parts.len() != 3 {
                    eprintln!("Please use format:YYY-Month-DD (e.g., 2000-January-15");
                } else {
                    let year = parts[0].parse::<u16>().unwrap_or(0);
                    let month = Month::from_str(parts[1]);
                    let day = parts[2].parse::<u8>().unwrap_or(0);
                    match month {
                        Ok(m) => match Birthday::new(year, m, day) {
                            Ok(bday) => {
                                current_user.birthday = Some(bday);
                                save_and_report(&current_user, args.global);
                            }
                            Err(e) => eprintln!("[{}]:{}", "ERROR".red(), e),
                        },
                        Err(e) => eprintln!("[{}]:{}", "ERROR".red(), e),
                    }
                }
            } else {
                println!("Key '{}' is not supported", args.key.yellow());
            }
        }
        Commands::Complation { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
        Commands::Gamemod => match gamemod::GamemMod::on() {
            Ok(_) => println!("game mode oned"),
            Err(e) => eprintln!("Error:{}", e),
        },
        Commands::Search {
            path,
            target,
            depth,
            block,
        } => {
            search(
                path.unwrap_or_else(|| current_dir().expect("i can't get current dir")),
                &target.unwrap_or_else(|| "".to_string()),
                depth.unwrap_or(0),
                block,
            );
        }
        Commands::ReadStory { path_file } => {
            readstory::read_file_story(path_file);
        }
        Commands::Disk => {
            monitoring::disk_check().await;
        }
    }
    Ok(())
}
