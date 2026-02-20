use std::str::FromStr;
use colored::Colorize;
use dirs::home_dir;
use std::fs;
use std::{fs::File, path::PathBuf};
use std::io::Write;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize,Deserialize,Debug)]
pub struct User{

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age:Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday:Option<Birthday>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email:Option<Email>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Email{
    pub email:String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Birthday{
    pub year:u16,
    pub month:Month,
    pub day:u8,
}
#[derive(Serialize,Deserialize,Debug)]
pub enum Month {
    January ,
    February,
    March ,
    April ,
    May ,
    June ,
    July ,
    August, 
    September ,
    October ,
    November ,
    December,
}

impl Birthday{
    pub fn new(year:u16,month:Month,day:u8)->Result<Self,String>{
        let max_day = match month{
            Month::January | Month::March | Month::May | Month::July | Month::August | Month::October|Month::December => 31,
            Month::November | Month::September|Month::June|Month::April => 30,
            Month::February=>28, 
        };
        if day < 1 || day > max_day{
            return Err(format!("Invalid day:{} for the selected month",day));
        }
        Ok(Self { year, month, day })
    }
}
impl Email {
    pub fn new(email:String)->Result<Self,String>{
        if !email.contains("@gmail.com"){
            return Err(String::from("please enter email"));
        }
        Ok(Self { email })
    }
}
impl FromStr for Month {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "january" | "jan" => Ok(Month::January),
            "february" | "feb"=> Ok(Month::February),
            "March" | "mar"=> Ok(Month::March),
            "april" | "apr"=> Ok(Month::April),
            "may" => Ok(Month::May),
            "june" | "jun"=> Ok(Month::June),
            "july" | "jul"=> Ok(Month::July),
            "august" | "aug"=> Ok(Month::August),
            "september" | "sep"=>Ok(Month::September),
            "octobr" | "oct"=> Ok(Month::October),
            "november"|"nov"=>Ok(Month::November),
            "december"|"dec"=>Ok(Month::December),
            _=> Err(format!("'{}' is not a valid month!",s.red())),
        }
    }
}

impl User {
    pub fn empty()->Self{
        Self { name: None, 
               age: None,  
               birthday: None, 
               email: None,
            }
    }
}
pub fn save_conf(user:&User,global:bool)->Result<(),Box<dyn std::error::Error>>{
    let path = if global{
        let mut p = home_dir().ok_or("cloud can't find home dir")?;
        p.push(".config/dex");
        fs::create_dir_all(&p)?;
        p.push("user.json");
        p
    }else{PathBuf::from(".dex.conf.json")};

    let json = serde_json::to_string_pretty(&user)?;
    let _ = fs::write(path, json);
    Ok(())
}
pub fn load_config(global:bool)->Result<User,Box<dyn std::error::Error>>{
    let  path = if global{
        home_dir()
        .map(|mut p | {p.push(".config/dex/user.json");p})
        .ok_or("cloud can't find home dir")?
    }else {
        PathBuf::from(".dex.conf.json")
    };
    if !path.exists(){
        return Ok(User::empty());
    }
    let content = fs::read_to_string(path)?;
    let user:User = serde_json::from_str(&content)?;
    Ok(user)

}
pub fn save_and_report(user: &User, global: bool) {
    if let Err(e) = save_conf(user, global) {
        eprintln!("[{}]: Failed to save config: {}", "ERROR".red(), e);
    } else {
        println!("[{}] Configuration updated successfully", "OK".green());
    }
}