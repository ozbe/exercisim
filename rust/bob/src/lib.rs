pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if is_silence(message) {
        "Fine. Be that way!"
    } else if is_yelling(message) {
        if is_question(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if is_question(message) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_yelling(message: &str) -> bool {
    message == message.to_uppercase() && message.chars().any(|c| c.is_alphabetic())
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_silence(message: &str) -> bool {
    message == ""
}
