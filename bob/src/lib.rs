/// Provide Bob's correct response to questions and statements
pub fn reply(message: &str) -> &str {
    let trimmed_message: &str = message.trim();
    let is_question: bool = trimmed_message.ends_with('?');
    let is_shouting: bool = trimmed_message.find(|x: char| x.is_uppercase()).is_some()
        && trimmed_message == trimmed_message.to_uppercase();
    let is_empty: bool = trimmed_message.is_empty();

    match (is_empty, is_question, is_shouting) {
        (true, false, false) => "Fine. Be that way!",
        (false, true, false) => "Sure.",
        (false, false, true) => "Whoa, chill out!",
        (false, true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever.",
    }
}
