pub fn factors(n: u64) -> Vec<u64> {
    let mut reducer = n;
    let mut current_divisor: u64 = 2;
    let mut prime_factors: Vec<u64> = [].to_vec();

    while reducer > 1 {
        if reducer % current_divisor == 0 {
            prime_factors.push(current_divisor);
            reducer /= current_divisor;
        } else {
            current_divisor += 1;
        }
    }

    prime_factors
}
