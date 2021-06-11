use regex::Regex;

pub fn reply(message: &str) -> &str {
    println!("::{}::", message.trim());
    let statement = Regex::new(r"\.$").unwrap();
    let end_with_whitespaces = Regex::new(r".*\?$").unwrap();
    let shouting = Regex::new(r"[A-Z]+.*$").unwrap();
    match (
        statement.is_match(message),
        end_with_whitespaces.is_match(message.trim()),
        shouting.is_match(message),
    ) {
        (true, false, false) => "Whatever.",
        (false, true, false) => "Sure.",
        (false, false, true) => "Whoa, chill out!",
        _ => "Not here",
    }
}
