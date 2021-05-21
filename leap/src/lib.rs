pub fn is_leap_year(year: u64) -> bool {
    let even_by_four = match year % 4 {
        0 => true,
        _ => false,
    };

    let odd_by_hundred_even_by_four_hundred: bool;

    if year % 100 != 0 || year % 400 == 0 {
        odd_by_hundred_even_by_four_hundred = true;
    } else {
        odd_by_hundred_even_by_four_hundred = false;
    }

    even_by_four == odd_by_hundred_even_by_four_hundred
}
