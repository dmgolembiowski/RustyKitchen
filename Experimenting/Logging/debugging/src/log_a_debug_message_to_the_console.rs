#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
/* Also required:
 * export 'RUST_LOG=debug cargo run'
