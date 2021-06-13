use regex::Regex;

pub fn reply(message: &str) -> &str {
    // println!("::{}::", message.trim());

    // let statement = Regex::new(r"\.$").unwrap();
    // let question = Regex::new(r".*\?$").unwrap();
    let numbers = Regex::new(r"[^a-zA-Z]").unwrap();
    // let shouting = Regex::new(r"[A-Z]+").unwrap();
    // let shouting_question = Regex::new(r".*[A-Z]+\?$").unwrap();
    if message.trim() == "" {
        return "Fine. Be that way!";
    };

    // if message.trim().ends_with(".") {
    //     return "Whatever.";
    // };

    if message.trim().ends_with("?") {
        return "Sure.";
    };

    // if shouting.is_match(message) {
    if message.trim().to_uppercase() == message.trim() {
        println!(
            "HELLO FROM IN HERE\n{}\n{}",
            message,
            message.to_uppercase()
        );
        if message.trim().ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        };
    };

    // if numbers.is_match(message) {
    //     return "Whatever";
    // }

    "Whatever."
}
