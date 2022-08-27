use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error_code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultSet {
    pub count: u16,
    pub first: Option<String>,
    pub last: Option<String>,
    pub played: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub id: u16,
    pub name: String,
    pub code: String,
    pub current_season: Option<Season>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub id: u32,
    pub current_matchday: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchCollection {
    pub result_set: ResultSet,
    pub matches: Vec<Match>,
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum MatchStatus {
    SCHEDULED,
    TIMED,
    IN_PLAY,
    PAUSED,
    FINISHED,
    SUSPENDED,
    POSTPONED,
    CANCELLED,
    AWARDED,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub id: u32,
    pub utc_date: DateTime<Utc>,
    pub competition: Competition,
    pub season: Season,
    pub stage: String,
    pub matchday: Option<u8>,
    pub status: Option<MatchStatus>,
    pub home_team: Team,
    pub away_team: Team,
    pub score: ScoreCard,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScoreCard {
    pub winner: Option<String>,
    pub half_time: Score,
    pub full_time: Score,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub home: Option<u8>,
    pub away: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub id: u16,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandingCollection {
    pub competition: Competition,
    pub standings: Vec<Standing>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Standing {
    pub table: Vec<Position>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub position: u8,
    pub team: Team,
    pub played_games: u8,
    pub won: u8,
    pub draw: u8,
    pub lost: u8,
    pub points: i8,
    pub goals_for: u8,
    pub goals_against: u8,
    pub goal_difference: i8,
}
