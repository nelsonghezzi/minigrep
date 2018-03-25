use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
    invert: bool,
}

impl Config {
    pub fn new(
        query: String,
        filename: String,
        case_sensitive: bool,
        invert: bool,
    ) -> Result<Config, &'static str> {
        Ok(Config {
            query,
            filename,
            case_sensitive,
            invert,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents, config.invert)
    } else {
        search_case_insensitive(&config.query, &contents, config.invert)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query) != invert)
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query) != invert)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_sensitive_invert_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "Pick three."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents, false)
        )
    }

    #[test]
    fn case_insensitive_invert_match() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search_case_insensitive(query, contents, true)
        )
    }
}
