pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    println!("{}", digits.len());

    match digits.map(|x| check_pow(x, digits.len())).sum() {
        0 => true,
        _ => false,
    }
}

fn check_pow(a: u32, b: usize) -> u32 {
    u32::pow(a, b as u32)
}
