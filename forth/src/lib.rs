pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<isize>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

use std::str::FromStr;

enum Token {
    Num(isize),
    Arith(&str),
    Mani(&str),
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![] }
    }

    pub fn format_stack(&self) -> String {
        let mut result = String::new();
        if self.stack.is_empty() {
            return result;
        }



        result
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let tokens: Vec<&str> = input.split(|x| !(x.is_alphabetic() ||
                                                  x.is_numeric() ||
                                                  "+*/-".contains(x)))
                                                  .collect();
        for tok in tokens {
            let tok = match isize::from_str(tok) {
                Ok(n) => Token::Num(n),
                Err(_) => {
                    let tok: String = tok.to_lowercase().collect();
                    match &tok {
                        "+" => {
                            let rhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(r) => r,
                            };
                            let lhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(l) => l,
                            };
                            self.stack.push(lhs + rhs);
                        },
                        "-" => {
                            let rhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(r) => r,
                            };
                            let lhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(l) => l,
                            };
                            self.stack.push(lhs - rhs);
                        },
                        "*" => {
                            let rhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(r) => r,
                            };
                            let lhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(l) => l,
                            };
                            self.stack.push(lhs * rhs);
                        },
                        "/" => {
                            let rhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(r) => r,
                            };
                            let lhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(l) => l,
                            };
                            if rhs == 0 {
                                return Err(Error::DivisionByZero);
                            }
                            self.stack.push(lhs / rhs);
                        },
                        "dup" => {
                            let rhs = match self.stack.pop() {
                                None => return Err(Error::StackUnderflow),
                                Some(r) => r,
                            };
                            self.stack.push(rhs);
                            self.stack.push(rhs);
                        },
                        "drop" => {},
                        "swap" => {},
                        "over" => {},
                    }
                },
            };
        }
        Ok(())
    }
}
