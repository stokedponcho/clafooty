use football_data::client::Client as FootballData;

use crate::application::mappers::map_competition;
use crate::configuration::Configuration;
use crate::domain::{Standing, StandingCollection};
use crate::scraper::Client as Scraper;

trait GetStandings {
    fn do_get_standings(&self, competition_id: u16) -> Option<StandingCollection>;
}

pub fn print_standings(configuration: Configuration, competition_ids: Vec<u16>) {
    let providers: Vec<Box<dyn GetStandings>> = vec![
        Box::new(Scraper {}),
        Box::new(FootballData::new(&configuration.token, None).unwrap()),
    ];

    competition_ids
        .iter()
        .flat_map(|competition_id| {
            for provider in providers.as_slice() {
                if let Some(collection) = provider.do_get_standings(*competition_id) {
                    return Some(collection);
                }
            }

            None
        })
        .for_each(|standings| {
            println!("{}", standings);
        });
}

impl GetStandings for Scraper {
    fn do_get_standings(&self, competition_id: u16) -> Option<StandingCollection> {
        Scraper::get_standings(competition_id)
    }
}

impl GetStandings for FootballData {
    fn do_get_standings(&self, competition_id: u16) -> Option<StandingCollection> {
        match self.get_standings(competition_id) {
            Ok(collection) => Some(StandingCollection {
                competition: map_competition(&collection.competition),
                table: collection.standings[0]
                    .table
                    .iter()
                    .map(|p| Standing {
                        position: p.position,
                        team: p.team.name.clone(),
                        played_games: p.played_games,
                        won: p.won,
                        draw: p.draw,
                        lost: p.lost,
                        points: p.points,
                        goals_for: p.goals_for,
                        goals_against: p.goals_against,
                        goal_difference: p.goal_difference,
                    })
                    .collect(),
            }),
            Err(football_data::dtos::ErrorMessage {
                error_code: 404,
                message: _,
            }) => None,
            Err(error) => panic!("{}", error.message),
        }
    }
}
