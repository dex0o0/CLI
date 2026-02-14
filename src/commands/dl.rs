use super::extractLinks::Link;
use anyhow::{anyhow,Result};
use dirs::download_dir;
use futures_util::StreamExt;
use indicatif::{ProgressBar,ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;


// main download fn
pub async fn download(url:&str)->Result<()>{
    let link = Link::new(url);
    
    if !link.is_valid(){
        return Err(anyhow!("link invalid:{}",url));
    }

    println!("link:{}",link.string);
    println!("category:{:?}",link.category);

    if let Some(extracted) = link.extract(){
        println!{"find:{}",extracted};
    }

    let filename = extract_filename(&link.string)?;
    let download_dir = download_dir()
        .ok_or(anyhow!("directory not found"))?;

    let filepath = download_dir.join(&filename);
    println!("save to:{}",filepath.display());

    download_file(&link.string,&filepath);
    println!("download completed:{}",filepath.display());
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







//download fn
async fn download_file(url:&str,path:&PathBuf)->Result<()>{
    let client = Client::builder()
    .user_agent("dex-cli/0.1.0")
    .build().unwrap_or_else(|e| {
        eprintln!("Error:{}",e);
        Client::new()
    });

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
    pb.finish_with_message("[*] download completed");
    Ok(())
    
}





