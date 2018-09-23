extern crate eos_api;
use eos_api::API;

fn main() {
    let eos_cli = API::new("https://api.eoslaomao.com");

    match eos_cli.get_currency_balance("eoslaomaocom", "eosio.token", "EOS") {
        Ok(info) => print!("{:#?}", info),
        Err(_) => println!("1111"),
    }
}
