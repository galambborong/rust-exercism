pub fn factors(n: u64) -> Vec<u64> {
    let mut reducer: u64 = n;
    let mut current_divisor: u64 = 2;
    let mut prime_factors: Vec<u64> = Vec::<u64>::new();

    while reducer > 1 {
        match reducer % current_divisor {
            0 => {
                prime_factors.push(current_divisor);
                reducer /= current_divisor;
            }
            _ => current_divisor += 1,
        }
    }

    prime_factors
}
