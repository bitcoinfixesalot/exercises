use std::panic;

const MULTIPLIED: &str = "multiplied";
const DIVIDED: &str = "divided";
const PLUS: &str = "plus";
const MINUS: &str = "minus";
const RAISED: &str = "raised";
const CUBED: &str = "cubed";

pub fn answer(command: &str) -> Option<i32> {
    let command = command.trim_end_matches('?');
    let words = parse_commands(command);

    if words.len() % 2 == 1 {
        let result = panic::catch_unwind(|| calculate(&words));
        match result {
            Ok(res) => return Some(res),
            Err(_) => return None,
        }
    }
    None
}

fn parse_commands(command: &str) -> Vec<&str> {
    command
        .split_whitespace()
        .filter(|&word| {
            word == MULTIPLIED
                || word == DIVIDED
                || word == PLUS
                || word == MINUS
                || word == RAISED
                || word == CUBED
                || word.parse::<i32>().is_ok()
        })
        .collect::<Vec<&str>>()
}

fn calculate(words: &Vec<&str>) -> i32 {
    let first = words.first().unwrap().parse::<i32>().unwrap();
    let num_iter = words
        .iter()
        .skip(2)
        .step_by(2)
        .map(|word| word.parse::<i32>().unwrap());
    let op_iter = words.iter().skip(1).step_by(2);
    num_iter
        .zip(op_iter)
        .fold(first, |acc, pair| match *pair.1 {
            PLUS => acc + pair.0,
            MINUS => acc - pair.0,
            DIVIDED => acc / pair.0,
            MULTIPLIED => acc * pair.0,
            RAISED => acc.pow(pair.0 as u32),
            _ => panic!("unsupported operation!"),
        })
}
