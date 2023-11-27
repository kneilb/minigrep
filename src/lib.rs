use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next(); // Skip app name

        let Some(query) = args.next() else {
            return Err("missing query");
        };
        let Some(filename) = args.next() else {
            return Err("missing filename");
        };

        // Case sensitive if the env var isn't defined.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|s| s.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_valid() {
        let args = ["app", "search", "file"].iter().map(|s| s.to_string());
        let config = Config::new(args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.query, "search");
        assert_eq!(config.filename, "file");
    }

    #[test]
    fn config_missing_filename() {
        let args = ["app", "search"].iter().map(|s| s.to_string());
        let config = Config::new(args);
        let Err(e) = config else {
            panic!();
        };
        assert_eq!(e, "missing filename");
    }

    #[test]
    fn config_missing_query_and_filename() {
        let args = ["app"].iter().map(|s| s.to_string());
        let config = Config::new(args);
        let Err(e) = config else {
            panic!();
        };
        assert_eq!(e, "missing query");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "u";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.,
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
