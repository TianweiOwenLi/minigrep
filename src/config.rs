
use std::{io, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
}


impl Config {
    pub fn new (args: &[String]) -> Result<Self, &'static str> {
        let (q, f) 
            = (args.get(1), args.get(2));
        
        match (q, f) {
            (Some(qs), Some(fs)) => 
                Ok(Self{query: qs.clone(), file_path: fs.clone()}),
            _ => 
                {
                    println!("Arguments: ");
                    for i in args {
                        print!("{i}, ");
                    }
                    println!("");
                    Err("Not enough arguments")
                },
        }
    }

    pub fn read_file(&self) -> io::Result<String> {
        fs::read_to_string(&self.file_path)
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }
}
