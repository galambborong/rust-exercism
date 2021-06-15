/// Check brackets are correctly matched and balanced
pub fn brackets_are_balanced(string: &str) -> bool {
    let chars: Vec<&str> = string.split("").filter(|x| !x.trim().is_empty()).collect();

    println!("{:?}, {}", chars, chars.len());

    match chars.len() {
        x if x == 0 => return true,
        x if x % 2 != 0 => return false,
        _ => return check_pairs(chars),
    }
}

fn check_pairs(chars: Vec<&str>) -> bool {
    match chars[0] {
        "[" => {
            if chars[chars.len() - 1] == "]" {
                return true;
            } else if chars[1] == "]" {
                return true;
            } else {
                return false;
            }
        }
        "{" => {
            if chars[chars.len() - 1] == "}" {
                return true;
            } else if chars[1] == "}" {
                return true;
            } else {
                return false;
            }
        }
        _ => false,
    }
}
