use std::{env, fs};
use std::env::Args;
use std::error::Error;

mod search;
mod cache;
mod iterator;
mod deref;
mod memory;
mod rc;
mod ref_demo;
mod tree;
mod thread_demo;
mod oop_demo;
pub mod server_demo;
pub mod thread_pool;


pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<'a>(mut args: Args) -> Result<Self, &'a str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        //println!("Searching for {:?}", query);
        //println!("In file {:?}", file_name);

        Ok(Self { query, file_name, case_sensitive })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let content: String = fs::read_to_string(&self.file_name)
            .map_err(|_err| {
                "something went wrong reading the file"
            })?;
        let results = if self.case_sensitive {
            search::search_case_sensitive(&self.query, &content)
        } else {
            search::search_case_insensitive(&self.query, &content)
        };

        for line in results.iter() {
            println!("{:?}", line);
        }

        Ok(())
    }
}


