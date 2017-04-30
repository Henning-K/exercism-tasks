use std::char;
use std::convert::From;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<'a> Luhn for &'a str  {
    fn valid_luhn(&self) -> bool {
        let code = self;
        if code.len() <= 1 || code.contains(|c| !char::is_numeric(c) && c != ' ') {
            return false;
        }
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

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}
