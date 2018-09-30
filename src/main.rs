extern crate eos_api;
#[macro_use]
extern crate clap;

use clap::App;
use eos_api::API;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let eos_cli = API::new("https://api.eoslaomao.com".to_string());

    if let Some(matches) = matches.subcommand_matches("info") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            match eos_cli.get_chain_info() {
                Ok(res) => println!("{:#?}", res),
                Err(error) => println!("{:#?}", error),
            }
        }
    }


    // match eos_cli.get_currency_balance("eoslaomaocom", "eosio.token", "EOS") {
    //     Ok(res) => println!("{:#?}", res),
    //     Err(error) => println!("{:#?}", error),
    // };

    // match eos_cli.get_connections() {
    //     Ok(res) => println!("{:#?}", res),
    //     Err(error) => println!("{:#?}", error),
    // }
}
