use football_data::client::Client;

use crate::application::mappers::map_competition;
use crate::domain::{Standing, StandingCollection};

pub fn print_standings(client: Client, competition_ids: Vec<u16>) {
    competition_ids
        .iter()
        .map(|competition_id| {
            client
                .get_standings(*competition_id)
                .unwrap_or_else(|error| panic!("{}", error.message))
        })
        .map(|s| StandingCollection {
            competition: map_competition(&s.competition),
            table: s.standings[0]
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
        })
        .for_each(|standings| {
            println!("{}", standings);
        });
}
