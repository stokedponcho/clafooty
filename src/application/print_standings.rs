use football_data::client::Client;

use crate::domain::{Competition, Standing, StandingCollection};

pub fn print_standings(client: Client, competition_ids: Vec<u16>) {
    competition_ids.into_iter().for_each(|competition_id| {
        let standings_dto = client
            .get_standings(competition_id)
            .unwrap_or_else(|error| panic!("{}", error.message));
        let standings = StandingCollection {
            competition: Competition {
                id: standings_dto.competition.id,
                name: standings_dto.competition.name,
                current_match_day: None,
            },
            table: standings_dto.standings[0]
                .table
                .iter()
                .map(|dto| Standing {
                    position: dto.position,
                    team: dto.team.name.clone(),
                    played_games: dto.played_games,
                    won: dto.won,
                    draw: dto.draw,
                    lost: dto.lost,
                    points: dto.points,
                    goals_for: dto.goals_for,
                    goals_against: dto.goals_against,
                    goal_difference: dto.goal_difference,
                })
                .collect(),
        };

        println!("{}", standings);
    });
}
