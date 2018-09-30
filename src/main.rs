extern crate eos_api;
use eos_api::API;

fn main() {
    let eos_cli = API::new("https://api.eoslaomao.com".to_string());

    match eos_cli.get_currency_balance("eoslaomaocom", "eosio.token", "EOS") {
        Ok(res) => println!("{:#?}", res),
        Err(error) => println!("{:#?}", error),
    };

    match eos_cli.get_chain_info() {
        Ok(res) => println!("{:#?}", res),
        Err(error) => println!("{:#?}", error),
    }

    match eos_cli.get_connections() {
        Ok(res) => println!("{:#?}", res),
        Err(error) => println!("{:#?}", error),
    }
}
