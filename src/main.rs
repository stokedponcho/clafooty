extern crate football_data;

mod application;
mod configuration;
mod display;
mod domain;

use application::print_current_fixtures::print_current_fixtures;
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
    let configuration = Configuration::new();
    let client = Client::new(&configuration.token).unwrap();

    match opts.subcmd {
        SubCommand::Matchday => print_current_fixtures(client, configuration.competitions),
        _ => panic!("Not implemented yet."),
    };

    Ok(())
}
