#[macro_use]
extern crate log;
extern crate syslog;

use syslog::{Facility, Error};

fn main() -> Result<(), Error> {
    syslog::init(Facility::LOG_USER,
                 log::LevelFilter::Debug,
                 Some("My app name"))?;
    debug!("this is a debug {}", "message");
    error!("this is an error!");
    Ok(())
}
