extern crate reqwest;

#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;
use reqwest::Client;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        IoError(::std::io::Error);
    }
}

fn run() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;

    let mut response = Client::new().post(paste_api).body(file).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;
    println!("Your paste is located at: {}", response_body);
    Ok(())
}

quick_main!(run);

