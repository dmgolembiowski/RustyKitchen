#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        error!("Failed to execute query: {}", err);
    }
}
