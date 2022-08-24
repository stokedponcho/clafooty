use crate::domain;

use super::mappers::map_competition;
use super::mappers::map_match;

pub fn print_current_fixtures(client: football_data::client::Client, competition_ids: Vec<u16>) {
    competition_ids
        .iter()
        .map(|c| {
            client
                .get_competition(*c)
                .unwrap_or_else(|error| panic!("{}", error.message))
        })
        .map(|dto| map_competition(&dto))
        .for_each(|competition| {
            let dto = client
                .get_competition_matches(competition.id, competition.current_match_day.unwrap())
                .unwrap_or_else(|error| panic!("{}", error.message));
            let matches: Vec<domain::Match> = dto
                .matches
                .iter()
                .map(|dto| -> domain::Match { map_match(dto) })
                .collect();
            let collection = domain::FixtureCollection {
                matches,
                competition,
            };

            println!("{}", collection);
        });
}
