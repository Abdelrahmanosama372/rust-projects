use std::env;
use std::process;
use quiz_maker::run;
use colored::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    if let Err(err) = run(args) {
        println!("{}",err.to_string().red());
        process::exit(1);
    }
}

