pub struct WordProblem {
    problem: String,
}

#[derive(Debug, PartialEq)]
enum Op {
    Plus,
    Minus,
    Div,
    Mul,
}

impl WordProblem {
    pub fn new(s: &str) -> Self {
        WordProblem { problem: s.to_string() }
    }

    pub fn answer(&self) -> Result<i64, &'static str> {
        if !self.problem.starts_with("What is ") || !self.problem.ends_with("?") {
            return Err("Given problem is malformed.");
        }
        let s = self.problem
                    .trim_left_matches("What is ")
                    .trim_right_matches("?")
                    .replace("plus", "+")
                    .replace("minus", "-")
                    .replace("multiplied by", "*")
                    .replace("divided by", "/");
        println!("{}", s);
        let mut ops: Vec<Op> = Vec::new();
        let mut nums: Vec<i64> = Vec::new();

        for m in s.split_whitespace() {
            match m.parse::<i64>() {
                Err(_) => {
                    match m {
                        "+" => ops.push(Op::Plus),
                        "-" => ops.push(Op::Minus),
                        "/" => ops.push(Op::Div),
                        "*" => ops.push(Op::Mul),
                        _ => return Err("Unknown operation."),
                    }
                }
                Ok(val) => {
                    if !ops.is_empty() && !nums.is_empty() {
                        let lhs = nums.pop().expect("Num stack is empty even after check.");
                        match ops.pop().expect("Op stack is empty even after check.") {
                            Op::Plus => {
                                nums.push(lhs + val);
                            }
                            Op::Minus => {
                                nums.push(lhs - val);
                            }
                            Op::Div => {
                                nums.push(lhs / val);
                            }
                            Op::Mul => {
                                nums.push(lhs * val);
                            }
                        }
                    } else {
                        nums.push(val);
                    }
                }
            }
        }
        nums.first()
            .map(|&e| e)
            .ok_or("No element on the number stack.")
    }
}
