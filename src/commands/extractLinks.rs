use regex::Regex;
use std::io::Result;
use tokio;
#[derive(Debug,Clone)]
pub struct Link{
    link:String,
    category:
}
#[derive(Debug,Clone,PartialEq)]
enum LinkCategory{
    Http,
    Https,
    Magnet,
    Torrent,
    File,
    Unknown,
}
impl Link{
    pub fn new(&mut self)->Self{
        let cat = Self::category(&self.url);
        Self{
            link:String::new(),
            category:cat,
        }
    }
    fn category(url:&str) ->LinkCategory {
        let patterns = [
            (r"^https?://",LinkCategory::Https),
            (r"^magnet:\?",LinkCategory::Magnet),
            (r"\.torrent$",LinkCategory::Torrent),
            (r"^(file|/|\./|\.\./)",LinkCategory::File),
        ]
        for (pattern,category) in patterns.iter(){
            let re = Regex::new{pattern}.expect{"Err from regex pattern"};
            if re.is_match(url){
                category.clone()
            }
        }
        LinkCategory::Unknown
    }

    pub fn is_valid(&self)-> bool{
        self.category != LinkCategory::Unknown
    }

    pub fn extract(&self) -> Option<String>{
        match self.category {
            LinkCategory::Http | LinkCategory::Https => {
                let re = Regex::new(r"https?://([^/]+)").ok()?;
                re.captures(&self.url)
                .map(|caps| caps[1].to_string())
            }
            LinkCategory::Magnet =>{
                let re = Regex::new(r"btih:([0-9a-fA-F])+").ok()?;
                re.captures(&self.url)
                .map(|caps| format!("Magnet:{}",&caps[1][..8]))
            }
            _ => None,
        }
    }
    
}