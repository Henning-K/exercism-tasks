use std::char;
use std::convert::From;

pub struct Luhn {
    valid: bool
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

impl<'a> From<&'a str> for Luhn  {
    fn from(code: &str) -> Self {
        if code.len() <= 1 || code.contains(|c| !char::is_numeric(c) && c != ' ') {
            return Luhn{valid: false};
        }
        Luhn { valid:
            code.chars()
                .filter(|&c| c.is_digit(10))
                .map(|c| c.to_digit(10)
                    .expect("Couldn't parse number from char."))
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
    }
}

impl From<String> for Luhn {
    fn from(code: String) -> Self {
        Self::from(code.as_str())
    }
}

impl From<usize> for Luhn {
    fn from(code: usize) -> Self {
        Self::from(code.to_string().as_str())
    }
}

impl From<u8> for Luhn {
    fn from(code: u8) -> Self {
        Self::from(code.to_string().as_str())
    }
}

impl From<u16> for Luhn {
    fn from(code: u16) -> Self {
        Self::from(code.to_string().as_str())
    }
}

impl From<u32> for Luhn {
    fn from(code: u32) -> Self {
        Self::from(code.to_string().as_str())
    }
}

impl From<u64> for Luhn {
    fn from(code: u64) -> Self {
        Self::from(code.to_string().as_str())
    }
}
