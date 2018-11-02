use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use eos_api::API;

pub fn fetch_command() -> App<'static, 'static> {
    App::new("get")
        .about("get info")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("info").about("get eos info"),)
        .subcommand(
            SubCommand::with_name("connections").about("get connections from eos blockchain"),
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("get balance from eos blockchain")
                .arg(
                    Arg::with_name("account")
                        .short("a")
                        .takes_value(true)
                        .required(true)
                        .help("....."),
                )
                .arg(
                    Arg::with_name("contract")
                        .short("c")
                        .takes_value(true)
                        .required(true)
                        .help("..."),
                )
                .arg(
                    Arg::with_name("symbol")
                        .short("s")
                        .takes_value(true)
                        .required(true)
                        .help("...."),
                ),
        )
        .subcommand(
            SubCommand::with_name("account")
                .about("get account info")
                .arg(
                    Arg::with_name("account")
                        .takes_value(true)
                        .required(true)
                        .help("...."),
                ),
        )
}

pub fn fetch_processor(matches: &ArgMatches) -> Result<(), String> {
    let eos = API::new("https://api.eoslaomao.com".to_string());
    match matches.subcommand() {
        ("info", _) => match eos.get_chain_info() {
            Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
            Err(error) => println!("{:#?}", error),
        },
        ("connections", _) => match eos.get_connections() {
            Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
            Err(error) => println!("{:#?}", error),
        },
        ("balance", Some(m)) => {
            let account = m.value_of("account").unwrap();
            let contract = m.value_of("contract").unwrap();
            let symbol = m.value_of("symbol").unwrap();
            println!("{}", account);
            println!("{}", contract);
            match eos.get_currency_balance(account, contract, symbol) {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(error) => println!("{:#?}", error),
            }
        }
        ("account", Some(m)) => {
            let account = m.value_of("account").unwrap();
            println!("{}", account);
            match eos.get_account(account) {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(error) => println!("{:#?}", error),
            }
        }
        ("accounts", Some(m)) => {
            let public_key = m.value_of("public_key").unwrap();
            match eos.get_accounts(public_key) {
                Ok(res) => println!("{}", serde_json::to_string_pretty(&res).unwrap()),
                Err(error) => println!("{:#?}", error),
            }
        }
        _ => {
            return Err(matches.usage().to_owned());
        }
    }
    Ok(())
}
