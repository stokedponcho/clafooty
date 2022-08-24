use football_data::dtos;

use crate::domain::{Competition, FixtureCollection, Match, MatchStatus, Score, ScoreCard};

pub fn map_fixtures(dto: dtos::MatchCollection) -> Vec<FixtureCollection> {
    dto.matches
        .iter()
        .map(|dto| &dto.competition)
        .fold(Vec::<&dtos::Competition>::new(), |mut acc, comp| {
            if acc.iter().all(|c| -> bool { c.id != comp.id }) {
                acc.push(comp);
            }
            acc
        })
        .iter()
        .map(|comp| FixtureCollection {
            count: dto.result_set.count as u8,
            competition: Competition {
                id: comp.id,
                name: comp.name.to_string(),
                current_match_day: match &comp.current_season {
                    Some(s) => Some(s.current_matchday),
                    None => None,
                },
            },
            matches: dto
                .matches
                .iter()
                .filter(|m| m.competition.name == comp.name)
                .map(|m| map_match(m))
                .collect(),
        })
        .collect()
}

pub fn map_match(dto: &dtos::Match) -> Match {
    let map_score = |dto: &dtos::Score| Score {
        home_team: dto.home,
        away_team: dto.away,
    };

    Match {
        utc_date: dto.utc_date,
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
    use chrono::{TimeZone, Utc};

    #[test]
    pub fn test_map_fixtures() {
        assert_eq!(
            vec![
                FixtureCollection {
                    count: 0,
                    competition: Competition {
                        id: 1,
                        name: "competition 1".to_string(),
                        current_match_day: Some(9),
                    },
                    matches: vec![create_match()],
                },
                FixtureCollection {
                    count: 0,
                    competition: Competition {
                        id: 2,
                        name: "competition 2".to_string(),
                        current_match_day: Some(10),
                    },
                    matches: vec![create_match()],
                },
            ],
            map_fixtures(dtos::MatchCollection {
                result_set: dtos::ResultSet {
                    count: 0,
                    first: "".to_string(),
                    last: "".to_string(),
                    played: 0
                },
                matches: vec![
                    create_dto_match_with_competition(dtos::Competition {
                        id: 1,
                        code: "CODE1".to_string(),
                        current_season: Some(dtos::Season {
                            id: 1,
                            current_matchday: 9,
                        }),
                        name: "competition 1".to_string(),
                    }),
                    create_dto_match_with_competition(dtos::Competition {
                        id: 2,
                        code: "CODE2".to_string(),
                        current_season: Some(dtos::Season {
                            id: 1,
                            current_matchday: 10,
                        }),
                        name: "competition 2".to_string(),
                    }),
                ]
            })
        );
    }

    #[test]
    pub fn test_map_match() {
        assert_eq!(create_match(), map_match(&create_dto_match()))
    }

    fn create_match() -> domain::Match {
        domain::Match {
            utc_date: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
            status: Some(domain::MatchStatus::Finished),
            home_team: String::from("home"),
            away_team: String::from("away"),
            score: domain::ScoreCard {
                winner: Some(String::from("winner")),
                full_time: domain::Score {
                    away_team: Some(4),
                    home_team: Some(5),
                },
                half_time: domain::Score {
                    away_team: Some(1),
                    home_team: Some(0),
                },
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
        }
    }

    impl PartialEq<domain::FixtureCollection> for domain::FixtureCollection {
        fn eq(&self, other: &domain::FixtureCollection) -> bool {
            self.count == other.count
                && self.competition == other.competition
                && self.matches == other.matches
        }
    }

    impl PartialEq<domain::Competition> for domain::Competition {
        fn eq(&self, other: &domain::Competition) -> bool {
            self.id == other.id
                && self.name == other.name
                && self.current_match_day == other.current_match_day
        }
    }

    impl PartialEq<domain::Match> for domain::Match {
        fn eq(&self, other: &domain::Match) -> bool {
            self.utc_date == other.utc_date
                && self.status == other.status
                && self.home_team == other.home_team
                && self.away_team == other.away_team
                && self.score == other.score
        }
    }

    impl PartialEq<domain::MatchStatus> for domain::MatchStatus {
        fn eq(&self, other: &domain::MatchStatus) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    impl PartialEq<domain::ScoreCard> for domain::ScoreCard {
        fn eq(&self, other: &domain::ScoreCard) -> bool {
            self.winner == other.winner
                && self.half_time == other.half_time
                && self.full_time == other.full_time
        }
    }

    impl PartialEq<domain::Score> for domain::Score {
        fn eq(&self, other: &domain::Score) -> bool {
            self.home_team == other.home_team && self.away_team == other.away_team
        }
    }
}
