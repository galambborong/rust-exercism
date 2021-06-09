use regex::Regex;

pub fn reply(message: &str) -> &str {
    let statement = Regex::new(r".$").unwrap();
    let end_with_whitespaces = Regex::new(r"\s+$").unwrap();
    match (
        statement.is_match(message),
        end_with_whitespaces.is_match(message),
    ) {
        (true, false) => "Whatever.",
        (false, true) => "Sure.",
        _ => "FAKE",
    }
}
