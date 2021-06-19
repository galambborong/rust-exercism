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
    let openings: Vec<&str> = vec!["[", "(", "{"];
    let closings: Vec<&str> = vec!["]", ")", "}"];
    let mut truthy: bool = false;
    let mut opening_bracket_locations: Vec<(usize, usize)> = Vec::new();
    let mut closing_bracket_locations: Vec<(usize, usize)> = Vec::new();
    for (brackets_idx, bracket) in brackets.iter().enumerate() {
        for (openings_idx, opening_bracket) in openings.iter().enumerate() {
            if bracket == opening_bracket {
                opening_bracket_locations.push((brackets_idx, openings_idx));
            }
        }
        for (closings_idx, closing_bracket) in closings.iter().enumerate() {
            if bracket == closing_bracket {
                closing_bracket_locations.push((brackets_idx, closings_idx));
            }
        }
    }

    println!("{:?}", opening_bracket_locations);
    println!("{:?}", closing_bracket_locations);

    for (idx, bracket_type) in opening_bracket_locations.iter().enumerate() {
        println!("{}", idx);
        println!("{:?}", bracket_type);
        match closing_bracket_locations[idx] {
            (_, bracket_type) => truthy = true,
            _ => truthy = false,
        }
    }

    // if bracket_locations[1].0 != bracket_locations[0].0 + 1 {
    //     println!(
    //         "Expected {}. Got {}",
    //         closings[bracket_locations[0].0],
    //         brackets[bracket_locations[0].0 + 1]
    //     );
    //     if closings[bracket_locations[0].0] != brackets[bracket_locations[0].0 + 1] {
    //         truthy = false;
    //         return truthy;
    //     }
    // }

    let mut idx = opening_bracket_locations[opening_bracket_locations.len() - 1].0 + 1;

    for n in idx..brackets.len() {
        println!("{} <-", brackets[n]);
        if brackets[n] == closings[opening_bracket_locations[idx - 1].1] {
            println!("Inside if on line 33. idx = {}", idx);
            truthy = true;
            idx -= 1;
        } else {
            truthy = false;
        }
    }

    truthy
}
