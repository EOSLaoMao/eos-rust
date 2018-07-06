#[macro_use]
extern crate clap;

use clap::{App, Arg};
mod api;

fn main() {
    let a = api::API::new("https://api.eoslaomao.com".to_string(), false);
    // let res = match a.get_chain_info() {
    //     Ok(n) => println!("{:?}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // };

    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Lemons <leoms.wang@gmail.com>")
        (@subcommand balance =>
            (about: "controls testing features")
            (version: "0.1")
            (@arg account: -a --account +takes_value "Print test information verbosely")
            (@arg code: -c --code +takes_value "Print test information verbosely")
            (@arg symbol: -s --symbol +takes_value "Print test information verbosely")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();

    // (as below), requesting just the name used, or both at the same time
    if let Some(_matches) = matches.subcommand_matches("balance") {
        let mut code = "eosio.token";
        let mut symbol = "EOS";

        if let Some(c) = _matches.value_of("code") {
            code = c;
        }

        if let Some(s) = _matches.value_of("symbol") {
            symbol = s;
        }

        if let Some(account) = _matches.value_of("account") {
            let _balance = match a.get_currency_balance(account, code, symbol) {
                Ok(mut res) => println!("{:?}", res.text()),
                Err(err) => println!("{}", err),
            };
        };
    } else {
        println!("Printing normally...");
    }
}
