extern crate clap;
extern crate football_data;

use clap::{Parser, Subcommand};
use football_data::client::Client;

mod application;
mod configuration;
mod display;
mod domain;
mod scraper;

use application::print_current_fixtures::print_current_fixtures;
use application::print_standings::print_standings;
use application::print_today_fixtures::print_today_fixtures;
use configuration::Configuration;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(about = "Show current season's matchday for competition")]
    Matchday {
        #[clap(short, long, value_parser, value_name = "COMPETITION ID")]
        competitions: Vec<u16>,
        // defaults to latest matchday
        #[clap(short, long, value_parser, value_name = "MATCHDAY")]
        matchday: Option<u8>,
    },
    #[clap(about = "Show current season's standings as of today for a competition")]
    Standings {
        #[clap(short, long, value_parser, value_name = "COMPETITION ID")]
        competitions: Vec<u16>,
    },
    #[clap(about = "Shows today's matches for all available competitions")]
    Today,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    run(opts.command)
}

fn run(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let configuration = Configuration::new();
    let client = Client::new(&configuration.token, None).unwrap();

    match command {
        Command::Matchday {
            competitions,
            matchday,
        } => print_current_fixtures(configuration, competitions, matchday),
        Command::Standings { competitions } => print_standings(configuration, competitions),
        Command::Today => print_today_fixtures(client),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn matchday() {
        assert!(run(Command::Matchday {
            competitions: vec![2021],
            matchday: None
        })
        .is_ok());
        assert!(run(Command::Matchday {
            competitions: vec![2021],
            matchday: Some(1)
        })
        .is_ok());
    }

    #[test]
    #[ignore]
    fn standings() {
        assert!(run(Command::Standings {
            competitions: vec![2021]
        })
        .is_ok());
    }

    #[test]
    #[ignore]
    fn today() {
        assert!(run(Command::Today).is_ok());
    }
}
