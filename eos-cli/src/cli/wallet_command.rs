use clap::{App, AppSettings, ArgMatches, SubCommand};
use keys::generator::Generator;
use keys::generator::Random;
use keys::Network::Mainnet;

pub fn wallet_command() -> App<'static, 'static> {
    App::new("wallet")
        .about("wallet")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("create_key").about("generate eos key pair"))
}

pub fn wallet_processor(matches: &ArgMatches) -> Result<(), String> {
    let _rand = Random::new(Mainnet);
    match matches.subcommand() {
        ("create_key", _) => match _rand.generate() {
            Ok(res) => println!("{}", res),
            Err(_) => println!("do nothing"),
        },
        _ => {
            return Err(matches.usage().to_owned());
        }
    }
    Ok(())
}
