use std::env;
use std::error::Error;
use std::fs;
use std::vec::Vec;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // // --snip--
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone();
        // let filename = args[2].clone();

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a filename string"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// Search a query in the contents
///
/// # Examples
/// ```
/// let query = String::from("rust");
/// let contents = String::from("\
/// rust
/// trust
/// me
/// ");
/// assert_eq!(vec!["rust", "trust"], minigrep::search(&query, &contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        )
    }
}
