use std::fmt;

pub struct Roman {
    num: usize,
}

static ROMAN_MAP: [(usize, &'static str); 13] = [(1, "I"),
                                                 (4, "IV"),
                                                 (5, "V"),
                                                 (9, "IX"),
                                                 (10, "X"),
                                                 (40, "XL"),
                                                 (50, "L"),
                                                 (90, "XC"),
                                                 (100, "C"),
                                                 (400, "CD"),
                                                 (500, "D"),
                                                 (900, "CM"),
                                                 (1000, "M")];

impl Roman {
    fn new(x: usize) -> Self {
        Roman { num: x }
    }
}

impl From<usize> for Roman {
    fn from(x: usize) -> Self {
        Roman::new(x)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut start = self.num.clone();
        let mut result = String::new();
        for &(numeric, roman_numeral) in ROMAN_MAP.iter().rev() {
            while start >= numeric {
                result.push_str(roman_numeral);
                start -= numeric;
            }
        }
        write!(f, "{}", result)
    }
}
