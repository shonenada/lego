#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate log;

extern crate serde_derive;

mod server;
mod wasmer;

use crate::server::start_server;

fn main() {
    pretty_env_logger::init();
    start_server();
}
