use std::result::Result::Ok;
use fs2::{self, free_space};
use tokio::net::TcpStream;
use super::extractLinks::Link;
use anyhow::{Result, anyhow};
use colored::Colorize;
use dirs::download_dir;
use futures_util::StreamExt;
use indicatif::{ProgressBar,ProgressStyle};
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;


// main download fn
pub async fn download(url:&str)->Result<()>{
    let link = Link::new(url);
    
    if !link.is_valid(){
        return Err(anyhow!("link invalid:{}",url));
    }

    // if let Some(extracted) = link.extract(){
    //     println!{"name server:{}",extracted};
    // }

    let filename = extract_filename(&link.string)?;
    let download_dir = download_dir().ok_or(anyhow!("directory not found"))?;
    let filepath = download_dir.join(&filename);
    println!("{}...","downloading".green());
    println!("{}:{}","name".green(),filename);
    
    if DownloadChecker::check_all(&link.string, &filepath).await{
        download_file(&link.string,&filepath).await?;
    }
    
    // println!("save to:{}",filepath.display());
    println!("{}:{}","download completed".green(),filepath.display());
    Ok(())
}

pub async fn download_with_filename(url:&str,name:&str)->Result<()>{
       let link = Link::new(url);
    
    if !link.is_valid(){
        return Err(anyhow!("link invalid:{}",url));
    }

    // if let Some(extracted) = link.extract(){
    //     println!{"name server:{}",extracted};
    // }

    let filename = name.to_string();
    let download_dir = download_dir().ok_or(anyhow!("directory not found"))?;
    let filepath = download_dir.join(&filename);
    println!("{}...","downloading".green());
    println!("{}:{}","name".green(),filename);
    download_file(&link.string,&filepath).await.expect("Error");

    // println!("save to:{}",filepath.display());
    
    println!("{}:{}","download completed".green(),filepath.display());

    Ok(())
}

pub async fn dl_read_file(file:PathBuf)->Result<()>{
    let links = fs::read_to_string(file) 
    .expect("Error can't read file");
    for url in links.lines(){
        if url.is_empty(){
            continue;
        }
        let link = Link::new(url);
        if !link.is_valid(){
            return Err(anyhow!("link invalid:{}",url));
        }
        download(&link.string as &str).await.expect("Error to download");
        println!("\n");
    }
    Ok(())
}


//extract filename
fn extract_filename(url:&str)->Result<String>{
    url.split('/')
    .last()
    .filter(|s| !s.is_empty())
    .map(|s| s.to_string())
    .ok_or_else(|| anyhow!("can't find filename"))
}
/////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
/// 
pub fn client_build()->reqwest::Client{
        let agent = format!("dex-cli/0.1.0");
        Client::builder()
            .user_agent(agent)
            .build().unwrap_or_else(|e|{
                eprintln!("[{}] to create client:{}","ERROR".red(),e);
                Client::new()
            })
    
}
//check all error
pub struct DownloadChecker;
impl DownloadChecker {
    pub async fn check_all(url:&str,path:&PathBuf)->bool{
        if !Self::check_internet().await {
        eprintln!("connection lost");
        return false;
        }
        if !Self::check_url(&url).await {
            return false;
        }
        if !Self::check_file(&path) {
            return false;
        }
        if !Self::check_url_access(&url).await {
            return false;
        }
        if !Self::check_space(&url, &path).await {
            return false;
        }
        let client = client_build();
        #[warn(unused_variables)]
        if let Ok(_e) = Self::handle_redirect(&client, url).await {
            true
        } else {
            false
        }
    }
    async fn handle_redirect(client:&reqwest::Client,url:&str)->Result<String>{
        let mut current_url = url.to_string();
        let mut visited = std::collections::HashSet::new();
        visited.insert(current_url.clone());
        for _ in 0..10{
            let response = client.head(&current_url).send().await?;
            if let Some(location) = response.headers().get("location"){
                let next_url = location.to_str().expect("Error").to_string();
                if visited.contains(&next_url){
                    eprintln!("[{}]redirect loop","ERROR".red());
                }
                visited.insert(next_url.clone());
                current_url = next_url;
            }else {
                break;
            }

        }
        Ok(current_url)
    }
    async fn check_url(url:&str)->bool{
        let mut out = true;
        if url.is_empty() {
            eprintln!("[{}] {} not found","ERROR".red(),"URL".blue());
            out = false;
        }
        
        if !url.starts_with("http://") && !url.starts_with("https://"){
            eprintln!("[{}] URL {}","ERROR".red(),"invalid".blue());
            out = false;
        }
        out
    }
    fn check_file(path:&PathBuf)->bool{
        if path.exists(){
            eprintln!("[{}] file exists ","ERROR".yellow());
            return false;
        }
        if let Some(parent) = path.parent(){
            if !parent.exists(){
                let _ = std::fs::create_dir_all(parent);
            }
            if !parent.is_dir(){
                eprintln!("{}","path file invalid".red());
                return false;
            }
        }
        
        true
    }
    async fn check_space(url:&str,path:&PathBuf)->bool{
        let client = client_build();
        let response = client.get(url).send().await.expect("Error");
        let total_size = response.content_length().unwrap_or(0);
        if let Some(parent) = path.parent() {
            
            match free_space(&parent){
                Ok(size)=>{
                    if size < total_size{
                        eprintln!("[{}] no space\n  needed:{}\n   avaliable:{}",
                                "ERROR".red(),
                                bytes_to_human(total_size),
                                bytes_to_human(size)
                        );
                        return false;
                    }else {
                        println!("[{}]:{:.2}MB","Size".green(),bytes_to_human(total_size));
                        return true;
                    }
                }
                Err(e)=>{
                    eprintln!("[{}] in check space\n{}","ERROR".red(),e);
                    return  false;
                }
            }        
        }else{eprintln!("path invalid");false}
            
    }
    async fn check_internet()->bool{
        TcpStream::connect("8.8.8.8:53").await.is_ok()
    }
    async fn check_url_access(url:&str)->bool{
        let mut bool =true;
        let client =Client::builder().timeout(std::time::Duration::from_secs(3)).build().expect("Error");
        let response = client.head(url).send().await.expect("Error");
        if !response.status().is_success(){
            eprintln!("HTTP {}",response.status());
            bool = false;
        }
        bool
    }


}

fn bytes_to_human(byte:u64)->f32{
    let human_size = byte as f32 / 1024.0 /1024.0;
    human_size
}

//download fn
async fn download_file(url:&str,path:&PathBuf)->Result<()>{
    let client = client_build();
    let response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"));
    let mut file = File::create(path).unwrap();
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await{
        let chunk = chunk?;
        file.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }
    Ok(())    
}





