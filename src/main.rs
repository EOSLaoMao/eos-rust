extern crate reqwest;
#[macro_use]
extern crate serde_json;

mod api;
use api::API;

fn main() {
    let eos = API::new("https://api.eoslaomao.com");
    let info = eos.get_chain_info();
    println!("{:#?}", info);
    let balance = eos.get_currency_balance("eoslaomaocom", "eosio.token", "EOS");
    println!("{:#?}", balance);
    let account = eos.get_account("eoslaomaocom");
    println!("{:#?}", account);
    let block = eos.get_block(100);
    println!("{:#?}", block);
}
