use std::fs;
use std::error::Error;
use colored::*;


pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Error: file path is missing".into());
    }

    let content = read_file(&args[1]);
    if let Err(err) = content {
        return Err(err);
    }

    let lines: Vec<String> = content.unwrap().split('\n').map(String::from).collect();
    display_question(&lines);

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn display_question(question: &[String]) -> () {
    println!("{}",question[0].bright_blue());
    for str in &question[1..] {
        println!("{}",str.green());
    }
}
