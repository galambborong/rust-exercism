pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut sum_of_pows = 0;
    let n = digits.len() as u32;

    for digit in digits {
        sum_of_pows += check_pow(digit, n);
    }

    sum_of_pows == num
}

fn check_pow(a: u32, b: u32) -> u32 {
    u32::pow(a, b)
}
