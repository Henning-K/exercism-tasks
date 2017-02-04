extern crate rand;

use rand::{thread_rng, Rng};

use std::char;

pub struct Robot {
    name: String,
}

impl Robot {
    fn gen_name() -> String {
        let mut rng = thread_rng();
        let mut name = String::new();

        #[allow(non_snake_case)]
        let (A, Z) = ('A' as u8, 'Z' as u8);
        for _ in 0..2 {
            name.push(rng.gen_range(A, Z + 1) as char);
        }
        for _ in 0..3 {
            name.push(char::from_digit(rng.gen_range(0, 10), 10).unwrap());
        }
        name
    }

    pub fn new() -> Self {
        Robot { name: Self::gen_name() }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let mut new_name = self.name.clone();
        while new_name == self.name {
            new_name = Self::gen_name();
        }
        self.name = new_name;
    }
}
