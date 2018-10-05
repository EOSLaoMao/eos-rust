extern crate eos_api;
#[macro_use]
extern crate clap;

use clap::App;
use eos_api::API;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let eos = API::new("https://api.eoslaomao.com".to_string());

    match matches.subcommand_name() {
        Some("info") => match eos.get_chain_info() {
            Ok(res) => println!("{:#?}", res),
            Err(error) => println!("{:#?}", error),
        },

        Some("connections") => match eos.get_connections() {
            Ok(res) => println!("{:#?}", res),
            Err(error) => println!("{:#?}", error),
        },

        Some("balance") => match matches.subcommand_matches("balance") {
            Some(_balance) => {
                let account = _balance.value_of("account").unwrap();
                let contract = _balance.value_of("contract").unwrap();
                let symbol = _balance.value_of("symbol").unwrap();

                match eos.get_currency_balance(account, contract, symbol) {
                    Ok(res) => print!("{:#?}", res),
                    Err(error) => println!("{:#?}", error),
                }
            }
            _ => (),
        },
        _ => println!("Don't be crazy"),
    }
}
