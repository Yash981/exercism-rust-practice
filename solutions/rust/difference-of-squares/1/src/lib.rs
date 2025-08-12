pub fn square_of_sum(n: u32) -> u32 {
    let mut square_sum = 0;
    for i in 1..=n {
        square_sum += i
    }
    square_sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_squares = 0;
    for i in 1..=n {
        sum_squares += i.pow(2);
    }
    sum_squares
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n) as u32
}
