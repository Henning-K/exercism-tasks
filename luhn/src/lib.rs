use std::char;

pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 || code.contains(|c| !char::is_numeric(c) && c != ' ') {
        return false;
    }
    code.chars()
        .filter(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).expect("Couldn't parse number from char."))
        .rev()
        .zip(1..)
        .map(|(val, i)| {
            let mut val = val as u64;
            if i % 2 == 0 {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            val
        })
        .fold(0, |acc, x| acc + x) % 10 == 0
}