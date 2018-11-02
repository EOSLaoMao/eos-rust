mod fetch_command;
mod wallet_command;
pub use self::fetch_command::{fetch_command, fetch_processor};
pub use self::wallet_command::{wallet_command, wallet_processor};

use clap::{App, AppSettings, ArgMatches};

pub fn build_cli() -> ArgMatches<'static> {
	App::new("eos")
		.version("0.1")
		.author("Lemons <wangqianyi.cs@gmail.com>")
		.about("Does awesome things")
		.global_setting(AppSettings::ColoredHelp)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommand(fetch_command())
		.subcommand(wallet_command())
		 .get_matches()
}
