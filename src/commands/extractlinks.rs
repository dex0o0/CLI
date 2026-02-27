use regex::Regex as Re;



#[derive(Debug,Clone)]
pub struct Link{
    pub string:String,
    pub category:LinkCategory,
}


#[derive(Debug,Clone,PartialEq)]
pub enum LinkCategory{
    Http,
    Https,
    Magnet,
    Torrent,
    File,
    Unknown,
}
impl Link{
    pub fn new(url:&str) -> Self{
        let categor = Self::category(&url);
        Link{
            string: url.to_string(),
            category:categor,
        }
    }
    fn category(url:&str) ->LinkCategory {
        let patterns = [
            (r"^https?://",LinkCategory::Https),
            (r"^magnet:\?",LinkCategory::Magnet),
            (r"\.torrent$",LinkCategory::Torrent),
            (r"^(file|/|\./|\.\./)",LinkCategory::File),
        ];
        for (pattern,category) in patterns.iter(){
             let re = Re::new(pattern).unwrap();
            if re.is_match(url){
                return category.clone()
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
                let re = Re::new(r"https?://([^/]+)").ok()?;
                re.captures(&self.string)
                .map(|caps| caps[1].to_string())
            }
            LinkCategory::Magnet =>{
                let re = Re::new(r"btih:([0-9a-fA-F])+").ok()?;
                re.captures(&self.string)
                .map(|caps| format!("Magnet:{}",&caps[1][..8]))
            }
            _ => None,
        }
    }
    
}