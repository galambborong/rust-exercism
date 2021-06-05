pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        64..=u32::MAX => panic!("Square must be between 1 and 64"),
        _ => square_method(s),
    }
}

fn square_method(x: u32) -> u64 {
    let mut square_total: u64 = 1;
    for _i in 1..x {
        square_total += square_total;
    }
    square_total
}

pub fn total() -> u64 {
    unimplemented!();
}
