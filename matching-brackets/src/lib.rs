use regex::Regex;

/// Check all brackets are correctly matched and balanced
pub fn brackets_are_balanced(string: &str) -> bool {
    let square_pair = Regex::new(r".*\[.*\].*").unwrap();
    let bracket_pair = Regex::new(r".*\(.*\).*").unwrap();
    let curly_pair = Regex::new(r".*\{.*\}.*").unwrap();

    let string_contains_square_pair = square_pair.is_match(string);
    let string_contains_bracket_pair = bracket_pair.is_match(string);
    let string_contains_curly_pair = curly_pair.is_match(string);
    let string_contains_no_pairs_but_is_balanced = string.is_empty();

    matches!(
        (
            string_contains_square_pair,
            string_contains_bracket_pair,
            string_contains_curly_pair,
            string_contains_no_pairs_but_is_balanced,
        ),
        (true, _, _, false)
            | (_, true, _, false)
            | (_, _, true, false)
            | (false, false, false, true)
    )
}
