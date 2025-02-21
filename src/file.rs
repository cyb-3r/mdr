use regex::Regex;
use std::path;

fn is_valid_filename(filename: &str) -> bool {
    let regex = Regex::new(r"^(?:(?:[\w]\:|\/)|(?:\.|\.\.))(\/[a-z_\-\s0-9\.]+)+\.(md|html)$")
        .expect("Failed to compile regex");
    regex.is_match(filename.trim())
}

pub fn read_path(path: &str) -> Result<path::PathBuf, std::io::Error> {
    return if is_valid_filename(path) {
        Ok(path::PathBuf::from(path))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path",
        ))
    };
}
