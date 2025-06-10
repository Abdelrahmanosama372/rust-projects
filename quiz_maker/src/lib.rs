use colored::*;
use std::error::Error;
use std::fs;
use std::io;

pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Error: file path is missing".into());
    }

    let content = read_file(&args[1])?;

    let lines: Vec<&str> = content.split('\n').filter(|s| !s.is_empty()).collect();

    let mut correct_ans = 0;
    for question in lines.chunks(6) {
        display_question(question);

        // ask user for answer
        let mut usr_ans = String::new();
        io::stdin().read_line(&mut usr_ans).unwrap();

        let res = check_question_answer(usr_ans.as_str(), question[5]);
        if res {
            correct_ans += 1;
            println!("{}", "correct".green());
        } else {
            println!("{}", "false".red());
        }
    }

    println!("{}", "------------------------------");
    println!("{} {}/{}", "your score:", correct_ans, lines.len() / 6);
    println!("{}", "------------------------------");

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn display_question(question: &[&str]) {
    println!("{}", question[0].magenta());
    for str in &question[1..5] {
        println!("{}", str.bright_cyan());
    }
}

fn check_question_answer(usr_ans: &str, answer: &str) -> bool {
    if answer.starts_with(usr_ans.trim()) {
        return true;
    }
    false
}
