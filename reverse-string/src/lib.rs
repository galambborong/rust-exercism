pub fn reverse(input: &str) -> String {
    let word: &str = input;
    let reversed_word: String = word.chars().rev().collect::<String>();
    reversed_word
}
