pub fn sum_of_multiples(num: u32, divs: &[u32]) -> u32 {
    (0..num)
        .filter(|x| divs.iter().any(|&n| x % n == 0))
        .sum()
}