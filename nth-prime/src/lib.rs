pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i = 2;
    while primes.len() < n as usize + 1 {
        if is_prime(i) {
            primes.push(i)
        }
        i += 1;
    }
    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    n != 1
}
