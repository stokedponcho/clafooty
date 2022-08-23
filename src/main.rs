extern crate football_data;

mod application;
mod configuration;
mod display;
mod domain;

use application::print_current_fixtures::print_current_fixtures;
use application::print_standings::print_standings;
use application::print_today_fixtures::print_today_fixtures;
use clap::Parser;
use configuration::Configuration;
use football_data::client::Client;

#[derive(Parser)]
enum SubCommand {
    Matchday,  // List current season's current day for favourite competitions.
    Today,     // List today's matches.
    Standings, // Show current season's standings as of date
}

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    run(opts.subcmd)
}

fn run(command: SubCommand) -> Result<(), Box<dyn std::error::Error>> {
    let configuration = Configuration::new();
    let client = Client::new(&configuration.token, None).unwrap();

    match command {
        SubCommand::Matchday => print_current_fixtures(client, configuration.competitions),
        SubCommand::Standings => print_standings(client, configuration.competitions),
        SubCommand::Today => print_today_fixtures(client),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn matchday() {
        assert!(run(SubCommand::Matchday).is_ok());
    }

    #[test]
    #[ignore]
    fn standings() {
        assert!(run(SubCommand::Standings).is_ok());
    }

    #[test]
    #[ignore]
    fn today() {
        assert!(run(SubCommand::Today).is_ok());
    }
}
