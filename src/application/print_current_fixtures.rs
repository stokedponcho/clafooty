use crate::application::mappers::map_fixtures;

use super::mappers::map_competition;

pub fn print_current_fixtures(
    client: football_data::client::Client,
    competition_ids: Vec<u16>,
    matchday: Option<u8>,
) {
    competition_ids
        .iter()
        .map(|competition_id| {
            let matchday = match matchday {
                Some(value) => value,
                None => {
                    let competition = client
                        .get_competition(*competition_id)
                        .unwrap_or_else(|error| panic!("{}", error.message));
                    let competition = map_competition(&competition);
                    competition.current_matchday.unwrap()
                }
            };

            client
                .get_competition_matches(*competition_id, matchday)
                .unwrap_or_else(|error| panic!("{}", error.message))
        })
        .map(|match_collection| map_fixtures(match_collection))
        .flatten()
        .for_each(|fixture_collection| {
            println!("{}", fixture_collection);
        });
}
