use mdr::Config;

fn main() {
    let std_args: Vec<String> = std::env::args().collect();

    let conf = Config::fetch(&std_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = mdr::run(conf) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
