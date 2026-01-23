use std::{collections::HashMap, thread::sleep, time::Duration};
use memory_stats::memory_stats;
use machine_info::{Machine};
use serde::{Deserialize,Serialize};
use serde_json::{self};
use sysinfo::{System,RefreshKind};
use std::fs;
use colored::*;

#[derive(Deserialize,Debug,Serialize)]
pub struct Sysinfo{

    // pub name_user:String,
    pub name_os:String,
    pub os_vertion:String,
    pub kernel:String,
    pub hostname:String,
    pub memory_size_as_mb:u64,
    pub cpu_brand:String,
    pub physical_cores:u32,
    pub total_processor:u32,
    pub hyperthreading_enabled:bool,
    pub gpu:String,
    pub gpu_brand:String,
    pub gpu_memory_as_mb:u64,
    pub gpu_temperatuer_celsius:u32,
}

impl Sysinfo {
    pub fn new() -> Self{
        Sysinfo{
            
            // name_user:String::new(),
            os_vertion:String::new(),
            name_os:String::new(),
            hostname:String::new(),
            gpu:String::new(),
            gpu_brand:String::new(),
            cpu_brand:String::new(),
            kernel:String::new(),
            memory_size_as_mb:0,
            total_processor:0,
            physical_cores:0,
            hyperthreading_enabled:false,
            gpu_memory_as_mb:0,
            gpu_temperatuer_celsius:0,
        }
    }
    pub fn auto_fill(&mut self)-> Result<(),Box<dyn std::error::Error>>{
        
        //auto fill System info
        let sys_info = Machine::new().system_info();
        self.name_os = sys_info.os_name;
        self.cpu_brand = sys_info.processor.brand;
        self.hostname = sys_info.hostname;
        self.kernel = sys_info.kernel_version;
        self.memory_size_as_mb = sys_info.memory/1024/1024;
        self.gpu = sys_info.graphics.iter().map(|m| m.name.clone()).collect();
        self.gpu_memory_as_mb = sys_info.graphics.iter().map(|g| g.memory  /1024/1024).sum();
        let temp_sum:u32 = sys_info.graphics.iter().map(|g| g.temperature).sum();
        let gpu_count = sys_info.graphics.len() as u32;
        self.gpu_temperatuer_celsius = if gpu_count > 0 {temp_sum/gpu_count}else{0};
        self.gpu_brand = sys_info.graphics.iter().map(|g | g.brand.clone()).collect();
        self.os_vertion = sys_info.os_version;
        self.total_processor = CPUINFO::get_thread_core();
        self.physical_cores = CPUINFO::get_core();
        self.hyperthreading_enabled = if self.total_processor > self.physical_cores{true}else{false} ;
        
        //self.name_user = sys_info.distribution;
        
        //Save data in json
        // let _ = self.save_to_json("SystemIndo.json");
        Ok(())
    }
    #[allow(dead_code)]
    pub fn save_to_json(&self,filename:&str)->Result<(),Box<dyn std::error::Error>>{
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn display(&self) {
    // ایجاد HashMap برای مپ کردن نام فیلد به مقدار
    let field_map: HashMap<&str, String> = HashMap::from([
        ("name_os", self.name_os.clone()),
        ("kernel", self.kernel.clone()),
        ("hostname", self.hostname.clone()),
        ("memory_size_as_mb", self.memory_size_as_mb.to_string()),
        ("cpu_brand", self.cpu_brand.clone()),
        ("gpu", self.gpu.clone()),
        ("gpu_brand", self.gpu_brand.clone()),
        ("gpu_memory_as_mb", self.gpu_memory_as_mb.to_string()),
        ("gpu_temperatuer_celsius", self.gpu_temperatuer_celsius.to_string()),
    ]);

    let field_names = vec![
        "name_os", "kernel", "hostname", "memory_size_as_mb", 
        "cpu_brand", "gpu", "gpu_brand", "gpu_memory_as_mb", 
        "gpu_temperatuer_celsius"
    ];

    // حالا می‌تونی مپ کنی
    field_names.iter()
        .filter_map(|name| field_map.get(name).map(|value| (name, value)))
        .for_each(|(name, value)| println!("{}: {}", name.blue().bold() , value.green()));
}
}





//get CPU usage

#[derive(Debug)]
pub struct CPUINFO{
    sys:System,
}
impl CPUINFO{
    #[allow(dead_code)]
    pub fn new() -> Self {
        let refresh_kind=RefreshKind::everything();
        let sys = System::new_with_specifics(refresh_kind);
        Self { sys: sys }  
    }
    #[allow(dead_code)]
    pub fn get_cpu_usage(&mut self) -> f32{
        self.sys.refresh_all();

        sleep(Duration::from_millis(200));
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()

        
    }
    pub fn get_core()-> u32{
       
        let physical_core = sysinfo::System::physical_core_count();
        match physical_core {
            Some(count)=> count as u32,
            None=>todo!()
        }
    }
    pub fn get_thread_core()-> u32{
        let sys =System::new_all();
        let virtual_cores= sys.cpus().len() as u32;
        virtual_cores
    }
}







#[allow(dead_code)]
pub fn physical_mem_stats()-> Option<f64>{
    if let Some(usege) = memory_stats(){
        Some(( usege.physical_mem as f64  / 1024.0 ).try_into().unwrap())
        
    }else{
        None
    }
}
#[allow(dead_code)]
pub fn virtual_mem_stats()-> Option<f64>{
    if let Some(usege) = memory_stats(){
        Some((usege.virtual_mem as f64 / 1024.0 ).try_into().unwrap())
    }else{
        None
    }
}
#[allow(dead_code)]
pub fn total_mem_stats(phy:f64,vir:f64)-> Option<f64>{
    let total = phy + vir;
    Some(total)
}