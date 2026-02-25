use colored::Colorize;
use rayon::prelude::*;
use std::fs;
use std::path::{Path,PathBuf};


const BLACKLIST: &[&str] = &["/proc","/sys","dev","/run","var/cache","dosdevices","drive_c"];
fn is_blacklisted(path:&Path)->bool{
    BLACKLIST.iter().any(|&b| path.to_string_lossy().contains(b))
}

pub fn search(path:PathBuf,target:&str,depth:usize){
   if target.is_empty() && depth == 0{
       normal_search(path);
   }else if !target.is_empty() && depth == 0 {
       deep_search(path, target);
   }else if depth > 0 {
       deep_search_depth(path, target, depth,depth);
   } 
}
fn normal_search(path:PathBuf){
    if let Ok(entries) = fs::read_dir(&path){
        let paths:Vec<PathBuf>=entries
            .flatten()
            .map(|e| e.path())
            .collect();
        paths.into_par_iter().for_each(|path|{
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            if name == "%"{
                return;
            }
            if path.is_dir(){
               print!("{}\t",name.blue());
            }else {
                print!("{}\t",name.green());
            }
        });
    }
}
fn deep_search(path:PathBuf,target:&str){
    let d = 1;
    if check_err(&path, &d){
        return;
    }
    if let Ok(entries) = fs::read_dir(&path){
        let paths:Vec<PathBuf>=entries
            .flatten()
            .map(|e| e.path())
            .collect();
        paths.into_par_iter().for_each(|path|{
            if path.is_dir(){
               deep_search(path, target);    
            }else if let Some(name) = path.file_name().and_then(|n| n.to_str()){
                if name.contains(target){
                    let size = fs::metadata(&path).map(|f| f.len()).unwrap_or(0);
                    println!("[{}-{}]{:?}","FOUND".green(),size_human(&size).to_string().yellow(),path);
                }
            }
        });     
    }
}
fn deep_search_depth(path: PathBuf, target: &str, current_depth: usize, initial_depth: usize) {
    if check_err(&path, &current_depth) {
        return;
    }

    let level = initial_depth - current_depth;
    
    if let Ok(entries) = fs::read_dir(&path) {
        let mut paths: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .collect();

        paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for (i, path) in paths.iter().enumerate() {
            let is_last = i == paths.len() - 1;
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            let connector = if is_last { "└── " } else { "├── " };
            let indent = "│   ".repeat(level);

            if target.is_empty() || name.contains(target) {
                if path.is_dir() {
                    println!("{}{}{}{}", indent.dimmed(), connector.dimmed(), "[DIR] ".blue(), name.blue().bold());
                    deep_search_depth(path.clone(), target, current_depth - 1, initial_depth);
                } else {
                    let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                    println!("{}{}[{}-{}]: {}", 
                        indent.dimmed(), 
                        connector.dimmed(), 
                        "FILE".green(), 
                        size_human(&size).yellow(),
                        name.to_string().green()
                    );
                }
            } else if path.is_dir() {

                deep_search_depth(path.clone(), target, current_depth - 1, initial_depth);
            }
        }
    }
}
fn size_human(size:&u64)->String{
    let units:Vec<&str> = vec!["B","KB","MB","GB","TB"];
   let mut unit_index = 0;
   let mut display_size = *size as f32;

   while display_size >= 1024.0 && unit_index < units.len() - 1 {
       display_size /= 1024.0;
       unit_index +=1;
   }

   format!("{:.2}{}",display_size,units[unit_index])
    
}
fn check_err(path:&PathBuf,depth:&usize)->bool{
    let mut bool = false;
    if *depth ==  0{
        bool= true;
    }
    if is_blacklisted(path){
        bool =true;
    }
    if let Ok(metadata) = fs::symlink_metadata(path){
        if metadata.file_type().is_symlink(){
            bool= true;
        }
    }
    bool
}
