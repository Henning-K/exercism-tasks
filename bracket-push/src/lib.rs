#[derive(PartialEq)]
enum BracketType {
    Open,
    SquareOpen,
    CurlyOpen,
}

pub struct Brackets {
    seq: String,
}

impl Brackets {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Brackets { seq: s.into() }
    }

    pub fn are_balanced(&self) -> bool {
        let mut bracket_stack = Vec::new();
        for c in self.seq.chars() {
            match c {
                '(' => bracket_stack.push(BracketType::Open),
                '[' => bracket_stack.push(BracketType::SquareOpen),
                '{' => bracket_stack.push(BracketType::CurlyOpen),
                ')' => {
                    match bracket_stack.pop() {
                        None => {
                            return false;
                        }
                        Some(BracketType::Open) => {}
                        Some(_) => {
                            return false;
                        }
                    }
                }
                ']' => {
                    match bracket_stack.pop() {
                        None => {
                            return false;
                        }
                        Some(BracketType::SquareOpen) => {}
                        Some(_) => {
                            return false;
                        }
                    }
                }
                '}' => {
                    match bracket_stack.pop() {
                        None => {
                            return false;
                        }
                        Some(BracketType::CurlyOpen) => {}
                        Some(_) => {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        bracket_stack.is_empty()
    }
}

impl<'a> From<&'a str> for Brackets {
    fn from(s: &'a str) -> Self {
        Brackets::new(s)
    }
}
