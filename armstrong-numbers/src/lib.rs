pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let n = digits.len() as u32;

    digits.iter().map(|x| u32::pow(*x, n)).sum::<u32>() == num
}
