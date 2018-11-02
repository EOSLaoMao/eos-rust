extern crate clap;
extern crate eos_api;
extern crate keys;
extern crate serde_json;
extern crate yaml_rust;

mod cli;
use cli::{build_cli, fetch_processor,wallet_processor};

fn main() {

    let matches = build_cli();

    if let Err(_err) = match matches.subcommand() {
        ("get", Some(m)) => fetch_processor(m),
        ("wallet", Some(m)) => wallet_processor(m),
        _ => Ok(()),
    } {
        // do nothing now
    }
}
