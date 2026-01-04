use tokio::fs;
use super::scan_sys;
use std::{
    fs::{File, read, read_to_string, write}, io::{BufReader, Read}, path::Path, sync::{Arc,Mutex}, thread, time::Duration
};
use env;

struct Logs{
    log:Arc<Mutex<Vec<StackLog>>>,
    monitorng:bool,
}
impl Logs {
    fn new(&mut self)->Self{
        let count = cunt_line_log();
        for _ in count{
            
        }
    }
}
struct StackLog{
    path:&'static str,
    content:String,
    level:String,
    keyword: &'static str,
}
impl StackLog {
    pub fn new(&mut self)-> Self{
        StackLog{
            path:"/var/log",
            content:String::new(),
            level:String::new(),
            keyword:"Error",
         }
    }
    pub fn check(&mut self,word:String) -> Result<(),String>{
        match check_word_in_file(self.path, word){
            (true,line) => {
                todo!("fix {line}")
            },
            _=>todo!(),
        }
    }
}

pub fn monitoring_mode(){

}


pub fn add_path_log(name:&'static str,content:String){
    let path = env::current_exe().expect("Error to get path file").parent().expect("").to_path_buf();
    let path_log = path.join("log").join(name);
    if !path_log.exists(){
        File::create(&path_log).expect("Error to creat File");    
    }
    fs::write(&path_log, content);
    

}
fn cunt_line_log()->u32{
    let binding = env::current_exe().expect("Error can't get path exe");
    let binding= binding.parent().expect("Error can't get parent path exe");
    let path_log  = binding.join("log").join("path.log");

    if path_log.exists(){
        let mut count:u32 =0;
        for _ in read_to_string(path_log).expect("Error to read file").lines(){
            count += 1;
        };
        count
    }else {
        println!("log file not exsis");
        return 0;
    }
}

fn check_word_in_file(path:&str,word:String)-> (bool,u32){
    let content_file = read_log_file(path).expect("Error read");
    let mut not_ok = false;
    let mut num_line:u32 = 0;
    for line in content_file.lines(){
        if line.contains(&word){
            not_ok = true;
        }else {
            not_ok = false;
        }
        num_line += 1;
    };
    (not_ok,num_line)
    
}
#[allow(dead_code)]
fn read_log_file(path:&str)-> Result<String,String> {
    let file = File::open(path).expect("Error in read file");
    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    buffer.read_to_string(&mut content).expect("Error to convert to string file");
    Ok(content)
}