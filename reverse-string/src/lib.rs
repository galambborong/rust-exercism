pub fn reverse(input: &str) -> String {
    let reversed_word: String = input.chars().rev().collect::<String>();
    reversed_word
}
