use crate::domain::{FixtureCollection, Match, MatchStatus, Score, Standing, StandingCollection};
use chrono::{DateTime, Local};
use std::fmt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

impl fmt::Display for FixtureCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);

        write!(f, "{}", self.competition.name).ok();
        write!(f, " (").ok();
        write!(f, "{}", self.stage).ok();

        if let Some(matchday) = self.matchday {
            write!(f, " - matchday {}", matchday).ok();
        }

        writeln!(f, ")").ok();

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

impl fmt::Display for StandingCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.competition.name).ok();
        writeln!(
            f,
            "{:^8} {:^26} {:^8} {:^8} {:^8} {:^8} {:^4} {:^4} {:^4} {:^8}",
            "Position", "Club", "Played", "Won", "Drawn", "Lost", "GF", "GA", "GD", "Points"
        )
        .ok();

        self.table.iter().for_each(|position| {
            writeln!(f, "{}", position).ok();
        });

        Ok(())
    }
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:^8} {:<26} {:^8} {:^8} {:^8} {:^8} {:^4} {:^4} {:^4} {:^8}",
            self.position,
            self.team,
            self.played_games,
            self.won,
            self.draw,
            self.lost,
            self.goals_for,
            self.goals_against,
            format!("{:+}", self.goal_difference),
            self.points,
        )
    }
}
