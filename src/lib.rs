mod search;
mod cache;

use std::{env, fs};
use std::error::Error;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_name: &'a String,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_name = &args[2];
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        //println!("Searching for {:?}", query);
        //println!("In file {:?}", file_name);

        Ok(Self { query, file_name, case_sensitive })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let content: String = fs::read_to_string(self.file_name)
            .map_err(|err| {
                "something went wrong reading the file"
            })?;
        let results = if self.case_sensitive {
            search::search_case_sensitive(self.query, &content)
        } else {
            search::search_case_insensitive(self.query, &content)
        };

        for line in results {
            println!("{:?}", line);
        }

        Ok(())
    }
}
