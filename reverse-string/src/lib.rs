use unicode_segmentation::UnicodeSegmentation;

/// Reverses any input string
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
