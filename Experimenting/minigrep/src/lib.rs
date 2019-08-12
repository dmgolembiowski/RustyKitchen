use std::error::Error;
use std::fs;

pub struct Configuration {
    pub query: String,
    pub filename: String,
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
    
        Ok(Configuration { query, filename })
    }
}

pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}
