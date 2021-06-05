pub fn square(s: u32) -> u64 {
    let mut square_total: u64 = 1;
    for i in 1..s {
        square_total += square_total;
        println!("{}", i);
    }
    square_total
}

pub fn total() -> u64 {
    unimplemented!();
}
