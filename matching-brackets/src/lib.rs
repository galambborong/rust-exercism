/// Check brackets are correctly matched and balanced
pub fn brackets_are_balanced(string: &str) -> bool {
    let chars: Vec<&str> = string.split("").filter(|x| !x.trim().is_empty()).collect();

    // let brackets: Vec<&str> = vec!["[", "]"];
    // let curlies: Vec<&str> = vec!["{", "}"];
    // let squares: Vec<&str> = vec!["[", "]"];

    // let collection: Vec<Vec<&str>> = vec![brackets, curlies, squares];

    let openings: Vec<&str> = vec!["[", "(", "{"];
    let closings: Vec<&str> = vec!["]", ")", "}"];

    let pairs: Vec<Vec<&str>> = vec![openings, closings];

    // println!("{:?}, {}", chars, chars.len());
    // println!("{:?}, {}", collection[0][0], collection.len());

    match chars.len() {
        x if x == 0 => return true,
        x if x % 2 != 0 => return false,
        _ => return check_pairs(chars, pairs),
    }
}

fn check_pairs(brackets: Vec<&str>, collection: Vec<Vec<&str>>) -> bool {
    let mut truthy: bool = false;
    let mut bracket_locations: Vec<(usize, usize)> = Vec::new();
    // let mut bracket_location = (0, 0);
    for (brackets_idx, bracket) in brackets.iter().enumerate() {
        println!("{:?} - line 31", bracket);
        for (openings_idx, opening_bracket) in collection[0].iter().enumerate() {
            // println!("{:?} <<<", j);
            if bracket == opening_bracket {
                println!("Hello from iteration {}", openings_idx);
                bracket_locations.push((brackets_idx, openings_idx));
            }
        }

        // if brackets_idx == bracket_location.0 + 1 && bracket == &collection[1][bracket_location.1] {
        //     println!("Pair");
        // } else if brackets_idx == bracket_location.0 + 1
        //     && bracket != &collection[1][!bracket_location.1]
        // {
        //     println!("Not a pair!")
        // }
    }
    // truthy
    println!("{:?}", bracket_locations);

    true
}
