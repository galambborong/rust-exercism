pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total_sum: u32 = 0;
    for i in 1..limit {
        if factors
            .iter()
            .any(|x| if x > &0 { i % x == 0 } else { false })
        {
            total_sum += i;
        }
    }
    total_sum
}
