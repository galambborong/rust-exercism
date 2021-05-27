pub fn square_of_sum(n: u32) -> u32 {
    let mut missing_values: u32 = n;
    let mut total: u32 = 0;

    while missing_values > 0 {
        total += missing_values;
        missing_values -= 1;
    }

    total * total
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut missing_values: u32 = n;
    let mut total: u32 = 0;

    while missing_values > 0 {
        total += missing_values * missing_values;
        missing_values -= 1;
    }

    total
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
