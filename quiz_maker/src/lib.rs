use std::fs;
use std::io;
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
    let mut correct_ans = 0;
    for question in lines.chunks(6) {
        display_question(question);

        // ask user for answer
        let mut usr_ans = String::new();
        io::stdin().read_line(&mut usr_ans).unwrap();

        let res = check_question_answer(&usr_ans,&question[5]);
        if res == true {
            correct_ans += 1;
            println!("{}","correct".green());
        }else {
            println!("{}","false".red());
        }
    }

    println!("{}","------------------------------");
    println!("{} {}/{}","your score:",correct_ans,lines.len()/6);
    println!("{}","------------------------------");

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

fn display_question(question: &[String]) -> () {
    println!("{}",question[0].magenta());
    for str in &question[1..5] {
        println!("{}",str.bright_cyan());
    }
}

fn check_question_answer(usr_ans: &String,answer: &String) -> bool {
    if usr_ans.starts_with(answer) {
        return true; 
    }
    return false;
}
