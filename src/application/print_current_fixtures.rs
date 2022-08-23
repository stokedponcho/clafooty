use crate::{application::mappers::map_match, domain};

pub fn print_current_fixtures(client: football_data::client::Client, competition_ids: Vec<u16>) {
    let competitions: Vec<domain::Competition> = competition_ids
        .iter()
        .map(|c| {
            client
                .get_competition(*c)
                .unwrap_or_else(|error| panic!("{}", error.message))
        })
        .map(|dto| domain::Competition {
            id: dto.id,
            name: dto.name,
            current_match_day: Some(dto.current_season.unwrap().current_matchday),
        })
        .collect();

    competitions.iter().for_each(|competition| {
        let fixtures = client
            .get_competition_fixtures(competition.id, competition.current_match_day.unwrap())
            .unwrap_or_else(|error| panic!("{}", error.message));
        let matches: Vec<domain::Match> = fixtures
            .matches
            .iter()
            .map(|dto| -> domain::Match { map_match(dto) })
            .collect();

        let collection = domain::FixtureCollection {
            count: fixtures.count,
            matches,
        };

        println!("{}", competition.name);
        println!("{}", collection);
    });
}
