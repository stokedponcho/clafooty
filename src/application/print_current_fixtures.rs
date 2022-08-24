use crate::application::mappers::map_fixtures;

use super::mappers::map_competition;

pub fn print_current_fixtures(
    client: football_data::client::Client,
    competition_id: u16,
    matchday: Option<u8>,
) {
    let matchday = match matchday {
        Some(value) => value,
        None => {
            let competition = client
                .get_competition(competition_id)
                .unwrap_or_else(|error| panic!("{}", error.message));
            let competition = map_competition(&competition);
            competition.current_match_day.unwrap()
        }
    };

    let matches = client
        .get_competition_matches(competition_id, matchday)
        .unwrap_or_else(|error| panic!("{}", error.message));

    for f in map_fixtures(matches).as_slice() {
        println!("{}", f);
    }
}
