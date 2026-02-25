use std::{fs};
use std::result::Result::Ok;
use serde::{Serialize,Deserialize};
use serde_json;


#[derive(Debug,Serialize,Deserialize)]
pub struct GamemMod {
    on:bool,
}


impl GamemMod {
    pub fn on() -> Result<(),String>{
        
        match set_cpu_governer("performance"){
          Ok(_)=>Ok(()),
          Err(e)=> Err(format!("Error:{}",e)),
        }

    }
}


fn set_cpu_governer(mode:&str) -> std::result::Result<(),std::string::String> {
    let paths = fs::read_dir("/sys/devices/system/cpu/").unwrap();
    //println!("paths:{:?}",&paths);
    for path in paths{
        //println!("path:{:?}",path);
        let path = path.unwrap().path();
        //println!("/////////////{:?}",path);
        if path.file_name().unwrap().to_str().unwrap().starts_with("cpu"){
            let gov_path=path.join("cpufreq/scaling_governor");
            if gov_path.exists(){
                fs::write(&gov_path, mode).expect("can't write");
                println!("set {:?} to {}",gov_path,mode);
                
            } else {
               println!("not found");
               continue;
            }
        }
    }
    Ok(())
}













