use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Team {
    name: String,
    win: u8,
    loss: u8,
    draw: u8,
}

impl Team {
    pub fn new(name: &str) -> Team {
        Team {
            name: name.to_string(),
            win: 0,
            draw: 0,
            loss: 0,
        }
    }

    pub fn points(&self) -> u8 {
        self.win * 3 + self.draw
    }

    pub fn matches(&self) -> u8 {
        self.win + self.draw + self.loss
    }
}

enum Result {
    Win,
    Loss,
    Draw,
}

pub fn tally(match_results: &str) -> String {
    let reader = BufReader::new(Cursor::new(match_results));
    let mut table: HashMap<String, Team> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (team1_name, team2_name, result) = match line.split(';').collect::<Vec<&str>>()[..] {
            [v0, v1, v2] => (v0, v1, v2),
            _ => unreachable!(),
        };

        keep_score(team1_name, team2_name, result, &mut table);
    }
    format_result(table)
}

fn keep_score(
    team1_name: &str,
    team2_name: &str,
    result: &str,
    scores: &mut HashMap<String, Team>,
) {
    match result {
        "win" => {
            score_team(team1_name, Result::Win, scores);
            score_team(team2_name, Result::Loss, scores);
        }
        "draw" => {
            score_team(team1_name, Result::Draw, scores);
            score_team(team2_name, Result::Draw, scores);
        }
        "loss" => {
            score_team(team1_name, Result::Loss, scores);
            score_team(team2_name, Result::Win, scores);
        }
        _ => unreachable!(),
    }
}

fn score_team(team_name: &str, result: Result, scores: &mut HashMap<String, Team>) {
    let team = scores
        .entry(team_name.to_string())
        .or_insert(Team::new(team_name));
    match result {
        Result::Win => team.win += 1,
        Result::Draw => team.draw += 1,
        Result::Loss => team.loss += 1,
    }
}

fn format_result(scores: HashMap<String, Team>) -> String {
    let mut result = vec![format!(
        "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        "Team", "MP", "W", "D", "L", "P"
    )];

    let mut scores = scores.iter().map(|(_, team)| team).collect::<Vec<_>>();
    scores.sort();
    scores.sort_by(|a, b| b.points().cmp(&a.points()));
    for team in scores.iter() {
        result.push(format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            team.name,
            team.matches(),
            team.win,
            team.draw,
            team.loss,
            team.points()
        ));
    }

    result.join("\n")
}
