use std::env;
use std::error::Error;
use std::fs; // For reading the file
use dotenv::dotenv;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //* If an error type is to be returned, it will be returned automatically from this function (run)

    let results = if config.case_sensitive {
        search(&contents, &config.query)
    } else {
        case_insensitive_search(&contents, &config.query)
    };

    for line in results {
        println!("{:?}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        dotenv().ok(); // Initialise .env file
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        print!("{:?}", case_sensitive);
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn search<'a>(content: &'a str, search_word: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(search_word) {
            result.push(line);
        }
    }
    result
}
pub fn case_insensitive_search<'a>(content: &'a str, search_word: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&search_word.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line() {
        let sample_content = "\
        Hello\nOne, two, three\nCheck
        ";
        let search_word = "three";
        assert_eq!(vec!["One, two, three"], search(sample_content, search_word));
    }

    #[test]
    fn case_insensitive() {
        let query = "cHeCk";
        let content = "\
        Hello\nOne, two, three\nCheck
        ";
        assert_eq!(vec!["Check"], case_insensitive_search(content, query));
    }
}
