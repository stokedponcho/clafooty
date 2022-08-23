use chrono::{DateTime, Utc};

pub struct Competition {
    pub id: u16,
    pub name: String,
    pub current_match_day: u8,
}

pub struct FixtureCollection {
    pub count: u32,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
pub enum MatchStatus {
    Scheduled,
    InPlay,
    Paused,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug)]
pub struct Match {
    pub utc_date: DateTime<Utc>,
    pub status: Option<MatchStatus>,
    pub home_team: String,
    pub away_team: String,
    pub score: ScoreCard,
}

#[derive(Debug)]
pub struct ScoreCard {
    pub winner: Option<String>,
    pub half_time: Score,
    pub full_time: Score,
}

#[derive(Debug)]
pub struct Score {
    pub home_team: Option<u8>,
    pub away_team: Option<u8>,
}
