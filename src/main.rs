const CMD: &str = "mdr";
const AUTHOR: &str = "cyb-3r";
const VERSION: &str = "0.0.1";

mod file;
mod parser;

use parser::tokenizer::Tokenizer;

fn main() {
    let std_args = std::env::args().collect::<Vec<String>>();

    if std_args.len() < 2 {
        println!("Usage: {} <file> (output)", CMD);
        std::process::exit(1);
    }

    // yes, I will make a better CLI parser
    if std_args[1] == "--version" || std_args[1] == "-v" {
        println!("{} v{} by {}", CMD, VERSION, AUTHOR);
        std::process::exit(0);
    }

    if std_args[1] == "--help" || std_args[1] == "-h" {
        println!("Usage: {} <file> (output)", CMD);
        std::process::exit(0);
    }
    // ---

    let file_path = match file::read_path(&std_args[1]) {
        Ok(path) => path,
        Err(_) => {
            println!("Invalid file path!");
            std::process::exit(1);
        }
    };

    // let content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let content = std::fs::read_to_string(file_path).unwrap();
    println!("{}", content);

    println!("Tokenizer test:");
    // let mut tokenizer = Tokenizer::new(String::from_utf8(content).unwrap());
    // parser::tokenizer::test_tokenizer(&mut tokenizer);
}
