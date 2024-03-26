use std::error::Error;
use std::fs;

pub struct Config {
    file_path: String,
    query: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let config;
        if args.len() == 4 {
            config = Config {
                ignore_case: {
                    if args.len() == 4 && args[1].contains("i") {
                        true
                    } else {
                        false
                    }
                },
                query: args[2].clone(),
                file_path: args[3].clone(),
            };
        } else {
            config = Config {
                ignore_case: false,
                query: args[1].clone(),
                file_path: args[2].clone(),
            };
        }

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let vec;
    if config.ignore_case == true {
        vec = search_ignore_case(&config.query, &content);
    } else {
        vec = search(&config.query, &content);
    }
    for line in vec {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_ignore_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "are";
        let contents = "\
            hello world
how are you
it is a test.";
        assert_eq!(vec!["how are you"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "ArE";
        let contents = "\
            hello world
how are you
it is a test.";
        assert_eq!(vec!["how are you"], search_ignore_case(query, contents));
    }
}
