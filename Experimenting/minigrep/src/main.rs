use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Configuration {
    query: String,
    filename: String,
}

impl Configuration {
    fn new(args: &[String]) -> Result<Configuration, &'static str> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Configuration::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", &config.query);
    println!("In file: {}", &config.filename);

    run(config);

}

fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}
