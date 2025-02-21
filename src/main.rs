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

    let mut output_path = std::path::PathBuf::from("./output.html");
    if std_args.len() == 3 {
        output_path = match std_args[2].parse() {
            Ok(path) => path,
            Err(_) => {
                println!("Invalid output path!");
                std::process::exit(1);
            }
        };
    }

    println!("source: {}", file_path.display());
    println!("output: {}", output_path.display());

    let content = std::fs::read_to_string(file_path).expect("Failed to read file");

    println!("!! Parsing content:\n{}", content);

    println!("!! Parsed content:");
    for line in content.lines() {
        if line.starts_with('#') {
            println!("<h1>{}</h1>", line[2..].trim());
        } else if line.is_empty() {
            println!("<br>");
        } else {
            println!("<p>{}</p>", line);
        }
    }

    println!("Tokenizer test:");
    let tokenizer = Tokenizer::new(content);
    for token in tokenizer {
        println!("{}", token);
    }

    println!("Program finished successfully");
}
