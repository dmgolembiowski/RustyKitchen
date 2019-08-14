use std::error::Error;
use std::fs;
use std::env;

pub struct Configuration {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Configuration {
    pub fn new(args: &[String]) -> Result<Configuration, &'static str> {
        if args.len() < 3 {
            return Err("\n \
                Suggested usage: cargo run [string] [file location] \
            \n");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Configuration { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* The lifetime "<'a>" parameter is a placeholder
     * for indicating which argument 
     *
     * Recall that a "lifetime" is like a more intelligently-controlled
     * version of scoping, where a certain type's data endures (on tha stack?) so long
     * as its lifetime is not closed (has not been cleared from stack);
     *
     * In particular, The constraint we want to express in this signature is 
     * that all the references in the parameters and the return value 
     * must have the same lifetime (so that the returned value can be
     * properly used, until it is no longer needed and can be un-plated
     * from the stack).
     */
    // Iterate over the contents and 
    // vec![]
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = " \
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

