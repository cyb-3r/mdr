mod block;
use block::Block;
use std::{error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub dest_path: String,
}

impl Config {
    pub fn fetch(args: &[String]) -> Result<Config, &'static str> {
        let len = args.len() - 1;
        if len < 1 {
            return Err("not enough arguments, expected at least 1 arg");
        }

        let file_path = args[1].clone();
        let dest_path = if len > 1 {
            args[2].clone()
        } else {
            String::from(".")
        };

        Ok(Config {
            file_path,
            dest_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let doc: Block = Block::split_block(content);
    println!("{:#?}", doc);
    Ok(())
}
