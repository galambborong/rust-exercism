/// Check brackets are correctly matched and balanced
pub fn brackets_are_balanced(string: &str) -> bool {
    let chars: Vec<&str> = string.split("").filter(|x| !x.trim().is_empty()).collect();

    match chars.len() {
        x if x == 0 => return true,
        x if x % 2 != 0 => return false,
        _ => return check_pairs(chars),
    }
}

fn check_pairs(brackets: Vec<&str>) -> bool {
    let remaining_brackets = remove_matches(brackets);
    remaining_brackets.is_empty()
}

fn remove_matches(mut brackets: Vec<&str>) -> Vec<&str> {
    let matching_brackets: Vec<&str> = vec!["{}", "()", "[]"];
    // for (idx, bracket) in brackets.iter().enumerate() {
    for (idx, bracket) in brackets.iter().enumerate().step_by(2) {
        println!("{}", idx);

        let temporary_pair = [bracket, brackets[idx + 1]].concat();
        println!("{}", temporary_pair);
        if matching_brackets
            .iter()
            .any(|pair| pair.to_string() == temporary_pair)
        {
            brackets.remove(idx);
            // brackets.remove()
        }
    }
    // brackets
    matching_brackets
}

// if factors.iter().any(|x| x > &0 && i % x == 0) {
// total_sum += i;
// }
