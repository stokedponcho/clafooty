use chrono::{Date, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Error {
    pub error: String,
    pub error_code: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Collection<T> {
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Season {
    pub season_id: u16,
    pub name: String,
    pub is_current: u8,
    pub country_id: u8,
    pub start_date: String,
    pub end_date: String,
}
