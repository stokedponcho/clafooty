use football_data::dtos;

use crate::domain::{Competition, FixtureCollection, Match, MatchStatus, Score, ScoreCard};

pub fn map_fixtures(dto: dtos::MatchCollection) -> Vec<FixtureCollection> {
    let tupl = dto.matches.iter().fold(
        Vec::<(u16, Competition, Option<u8>, String)>::new(),
        |mut acc, game| {
            let comp = &game.competition;

            if acc
                .iter()
                .all(|t| !(t.0 == comp.id && t.2 == game.matchday && t.3 == game.stage))
            {
                acc.push((
                    comp.id,
                    map_competition(&comp),
                    game.matchday,
                    game.stage.clone(),
                ));
            }

            acc
        },
    );

    tupl.into_iter()
        .map(|group| {
            let competition = group.1;
            let matchday = group.2;
            let stage = group.3;

            FixtureCollection {
                competition: Competition {
                    id: competition.id,
                    name: competition.name.clone(),
                    current_matchday: competition.current_matchday,
                },
                matches: dto
                    .matches
                    .iter()
                    .filter(|m| {
                        m.competition.id == competition.id
                            && m.matchday == matchday
                            && m.stage == stage
                    })
                    .map(|m| map_match(m))
                    .collect(),
                matchday: matchday,
                stage: stage.clone(),
            }
        })
        .collect()
}

pub fn map_competition(dto: &dtos::Competition) -> Competition {
    Competition {
        id: dto.id,
        name: dto.name.to_string(),
        current_matchday: match &dto.current_season {
            Some(s) => Some(s.current_matchday),
            None => None,
        },
    }
}

pub fn map_match(dto: &dtos::Match) -> Match {
    let map_score = |dto: &dtos::Score| match (dto.home, dto.away) {
        (Some(home), Some(away)) => Some(Score {
            home_team: home,
            away_team: away,
        }),
        _ => None,
    };

    Match {
        date: Some(dto.utc_date.date_naive()),
        datetime: Some(dto.utc_date),
        status: match dto.status {
            Some(dtos::MatchStatus::FINISHED) => Some(MatchStatus::Finished),
            Some(dtos::MatchStatus::IN_PLAY) => Some(MatchStatus::InPlay),
            Some(dtos::MatchStatus::PAUSED) => Some(MatchStatus::Paused),
            Some(dtos::MatchStatus::POSTPONED) => Some(MatchStatus::Postponed),
            Some(dtos::MatchStatus::SCHEDULED) => Some(MatchStatus::Scheduled),
            Some(dtos::MatchStatus::TIMED) => Some(MatchStatus::Scheduled),
            Some(dtos::MatchStatus::CANCELLED) => Some(MatchStatus::Cancelled),
            _ => None,
        },
        home_team: dto.home_team.name.clone(),
        away_team: dto.away_team.name.clone(),
        score: ScoreCard {
            winner: dto.score.winner.clone(),
            half_time: map_score(&dto.score.half_time),
            full_time: map_score(&dto.score.full_time),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain;
    use chrono::{NaiveDate, TimeZone, Utc};

    #[test]
    pub fn test_map_fixtures() {
        assert_eq!(
            vec![
                FixtureCollection {
                    competition: Competition {
                        id: 1,
                        name: "competition 1".to_string(),
                        current_matchday: None,
                    },
                    matchday: Some(6),
                    stage: "STAGE".to_string(),
                    matches: vec![create_match()],
                },
                FixtureCollection {
                    competition: Competition {
                        id: 2,
                        name: "competition 2".to_string(),
                        current_matchday: None,
                    },
                    matchday: Some(6),
                    stage: "STAGE".to_string(),
                    matches: vec![create_match(), create_match()],
                },
            ],
            map_fixtures(dtos::MatchCollection {
                result_set: dtos::ResultSet {
                    count: 0,
                    first: None,
                    last: None,
                    played: None
                },
                matches: vec![
                    create_dto_match_with_competition(dtos::Competition {
                        id: 1,
                        code: "CODE1".to_string(),
                        current_season: None,
                        name: "competition 1".to_string(),
                    }),
                    create_dto_match_with_competition(dtos::Competition {
                        id: 2,
                        code: "CODE2".to_string(),
                        current_season: None,
                        name: "competition 2".to_string(),
                    }),
                    create_dto_match_with_competition(dtos::Competition {
                        id: 2,
                        code: "CODE2".to_string(),
                        current_season: None,
                        name: "competition 2".to_string(),
                    }),
                ],
            })
        );
    }

    #[test]
    pub fn test_map_match() {
        assert_eq!(create_match(), map_match(&create_dto_match()))
    }

    fn create_match() -> domain::Match {
        domain::Match {
            date: Some(NaiveDate::from_ymd(2000, 1, 1)),
            datetime: Some(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
            status: Some(domain::MatchStatus::Finished),
            home_team: String::from("home"),
            away_team: String::from("away"),
            score: domain::ScoreCard {
                winner: Some(String::from("winner")),
                full_time: Some(domain::Score {
                    away_team: 4,
                    home_team: 5,
                }),
                half_time: Some(domain::Score {
                    away_team: 1,
                    home_team: 0,
                }),
            },
        }
    }

    fn create_dto_match() -> dtos::Match {
        create_dto_match_with_competition(dtos::Competition {
            id: 1,
            code: "CODE".to_string(),
            current_season: Some(dtos::Season {
                id: 1,
                current_matchday: 1,
            }),
            name: "competition".to_string(),
        })
    }

    fn create_dto_match_with_competition(competition: dtos::Competition) -> dtos::Match {
        dtos::Match {
            id: 1,
            utc_date: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            competition,
            season: dtos::Season {
                id: 0,
                current_matchday: 10,
            },
            status: Some(dtos::MatchStatus::FINISHED),
            home_team: dtos::Team {
                id: 2,
                name: "home".to_string(),
            },
            away_team: dtos::Team {
                id: 1,
                name: "away".to_string(),
            },
            score: dtos::ScoreCard {
                winner: Some("winner".to_string()),
                full_time: dtos::Score {
                    away: Some(4),
                    home: Some(5),
                },
                half_time: dtos::Score {
                    away: Some(1),
                    home: Some(0),
                },
            },
            stage: "STAGE".to_string(),
            matchday: Some(6),
        }
    }
}
