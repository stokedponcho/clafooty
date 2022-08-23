use crate::domain::{FixtureCollection, Match, MatchStatus, Score};
use chrono::{DateTime, Local};
use std::fmt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

impl fmt::Display for FixtureCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);

        self.matches.iter().for_each(|game| {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                .ok();
            writeln!(f, "{}", game).ok();
        });

        stdout.reset().ok();
        Ok(())
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_score = |score: &Score, state| {
            format!(
                "{} - {} {}",
                score.home_team.unwrap(),
                score.away_team.unwrap(),
                state
            )
        };
        let score = match &self.status {
            Some(MatchStatus::Scheduled) => DateTime::<Local>::from(self.utc_date)
                .format("%a %d %B %H:%M")
                .to_string(),
            Some(MatchStatus::Finished) => format_score(&self.score.full_time, "(FT)"),
            Some(MatchStatus::InPlay) | Some(MatchStatus::Paused) => {
                format_score(&self.score.full_time, "")
            }
            Some(other) => format!("{:?}", other),
            _ => String::from("UNKWOWN"),
        };

        write!(f, "{:>26} {} {}", self.home_team, score, self.away_team)
    }
}
