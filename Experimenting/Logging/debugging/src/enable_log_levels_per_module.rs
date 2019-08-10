#[macro_use]
extern crate log;
extern crate env_logger;

// See: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html#enable-log-levels-per-module

mod foo {
    mod bar {
        pub fn run() {
            warn!("[bar] warn");
            info!("[bar] info");
            debug!("[bar] debug");
        }
    }

    pub fn run() {
        warn!("[foo] warn");
        info!("[foo] info");
        debug!("[foo] debug");
        bar::run();
    }
}

fn main() {
    env_logger::init();
    warn!("[root] warn");
    info!("[root] info");
    debug!("[root] debug");
    foo::run();
}

