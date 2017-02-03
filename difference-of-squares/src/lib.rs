pub fn sum_of_squares(n: u32) -> u32 {
    (1u32..n + 1).map(|e| e * e).sum()
}

pub fn square_of_sum(n: u32) -> u32 {
    (1..n + 1).fold(0u32, |sum, acc| sum + acc).pow(2)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
