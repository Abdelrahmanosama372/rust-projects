use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let vec: Vec<String> = env::args().collect();
    let config = Config::build(&vec).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}