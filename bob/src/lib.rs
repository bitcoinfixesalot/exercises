const FINE_BE_THAT_WAY: &str = "Fine. Be that way!";
const SURE: &str = "Sure.";
const WHATEVER: &str = "Whatever.";
const WHOA_CHILL_OUT: &str = "Whoa, chill out!";
const QUESTION_MARK: &str = "?";
const CALM_DOWN: &str = "Calm down, I know what I'm doing!";

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return FINE_BE_THAT_WAY;
    }

    match (is_question(message), is_yelling(message)) {
        (true, true) => CALM_DOWN,
        (true, false) => SURE,
        (false, true) => WHOA_CHILL_OUT,
        (false, false) => WHATEVER,
    }
}

fn is_yelling(message: &str) -> bool {
    is_alphabetic(message) && message == message.to_uppercase()
}

fn is_alphabetic(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic())
}

fn is_question(message: &str) -> bool {
    message.ends_with(QUESTION_MARK)
}
