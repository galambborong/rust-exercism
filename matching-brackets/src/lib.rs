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

fn check_pairs(chars: Vec<&str>, collection: Vec<Vec<&str>>) -> bool {
    let mut truthy: bool = false;
    for (i, el) in chars.iter().enumerate() {
        for j in &collection[i] {
            // println!("el = {:?}, el == j: {:?}", i, el == j);
            println!("el = {:?}, i = {:?}", el, i);
            if el == j {
                // if &collection[1][i];
                truthy;
            }
        }
    }
    truthy
}
