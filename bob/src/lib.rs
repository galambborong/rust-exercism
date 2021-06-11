use regex::Regex;

pub fn reply(message: &str) -> &str {
    println!("::{}::", message.trim());

    let statement = Regex::new(r"\.$").unwrap();
    let question = Regex::new(r".*\?$").unwrap();
    let shouting = Regex::new(r"[A-Z]+.*$").unwrap();
    let shouting_question = Regex::new(r"[A-Z]+\?$").unwrap();

    if statement.is_match(message) {
        return "Whatever.";
    };

    if question.is_match(message.trim()) {
        return "Sure.";
    };

    if shouting.is_match(message) {
        return "Whoa, chill out!";
    }
    if message.trim() == "" {
        return "Fine. Be that way!";
    }

    if shouting_question.is_match(message) {
        return "Calm down, I know what I'm doing!";
    }

    "Whatever."

    // match (
    //     statement.is_match(message),
    //     end_with_whitespaces.is_match(message.trim()),
    //     shouting.is_match(message),
    // ) {
    //     (true, false, false) => "Whatever.",
    //     (false, true, false) => "Sure.",
    //     (false, false, true) => "Whoa, chill out!",
    //     _ => "Not here",
    // }
}
