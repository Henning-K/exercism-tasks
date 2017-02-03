#[derive(PartialEq, Debug)]
enum Frame {
    Strike,
    Spare(u32, u32),
    Open(u32, u32),
    Tenth(Option<u32>, Option<u32>, Option<u32>),
}

pub struct BowlingGame {
    cur_frame_num: u8,
    current_turn: Option<u32>,
    rolls: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            cur_frame_num: 0,
            current_turn: None,
            rolls: vec![],
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<(), &str> {
        if self.cur_frame_num >= 10 {
            return Err("Game is already done.");
        }
        if pins > 10 {
            return Err("Can not roll more than 10 pins in a single roll.");
        }
        match (self.current_turn, pins) {
            (None, 10) => {
                // Strike
                if self.cur_frame_num == 9 {
                    // Tenth frame.
                    let temp = self.rolls.pop().unwrap();
                    match temp {
                        Frame::Tenth(Some(first), None, None) => {
                            self.rolls.push(Frame::Tenth(Some(first), Some(10), None));
                            return Ok(());
                        }
                        Frame::Tenth(Some(first), Some(second), None) => {
                            if first == 10 && second < 10 {
                                return Err("Final ball after a strike and non-strike can't be a \
                                            strike.");
                            }
                            self.rolls.push(Frame::Tenth(Some(first), Some(second), Some(10)));
                            self.cur_frame_num += 1;
                            return Ok(());
                        }
                        other => {
                            self.rolls.push(other);
                            self.rolls.push(Frame::Tenth(Some(10), None, None));
                            return Ok(());
                        }
                    }
                }
                self.rolls.push(Frame::Strike);
                self.cur_frame_num += 1
            }
            (None, n) => {
                // incomplete turn.
                if self.cur_frame_num == 9 {
                    let temp = self.rolls.pop().unwrap();
                    match temp {
                        Frame::Tenth(Some(first), None, None) => {
                            self.rolls.push(Frame::Tenth(Some(first), Some(n), None));
                            if first + n < 10 {
                                self.cur_frame_num += 1;
                            }
                            return Ok(());
                        }
                        Frame::Tenth(Some(first), Some(second), None) => {
                            if first == 10 && second < 10 && second + n > 10 {
                                return Err("Invalid number of pins after final strike.");
                            }
                            self.rolls.push(Frame::Tenth(Some(first), Some(second), Some(n)));
                            self.cur_frame_num += 1;
                            return Ok(());
                        }
                        other => {
                            self.rolls.push(other);
                            self.rolls.push(Frame::Tenth(Some(n), None, None));
                            return Ok(());
                        }
                    }
                }
                self.current_turn = Some(n);
            }
            (Some(i), j) => {
                // completion of turn.
                match i + j {
                    10 => {
                        self.cur_frame_num += 1;
                        self.current_turn = None;
                        self.rolls.push(Frame::Spare(i, j));
                    }
                    n => {
                        if n > 10 && self.cur_frame_num < 9 {
                            return Err("More than 10 pins per frame are only allowed in the last \
                                        frame.");
                        }
                        self.cur_frame_num += 1;
                        self.current_turn = None;
                        self.rolls.push(Frame::Open(i, j));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn score(&self) -> Result<u32, &str> {
        if self.rolls.len() < 10 {
            return Err("Not enough frames for scoring.");
        }

        match self.rolls[9] {
            Frame::Tenth(Some(10), None, None) => return Err("Ill-formed 10th frame."),
            Frame::Tenth(Some(10), Some(10), None) => return Err("Ill-formed 10th frame."),
            Frame::Tenth(Some(i), Some(j), None) if i + j == 10 => {
                return Err("Ill-formed 10th frame.")
            }
            _ => {}
        }

        let mut result = 0;

        for (ind, val) in self.rolls.iter().enumerate() {
            let val = val.clone();
            result += match *val {
                Frame::Strike => {
                    10 +
                    match self.rolls[ind + 1] {
                        Frame::Strike => {
                            10 +
                            match self.rolls[ind + 2] {
                                Frame::Strike => 10,
                                Frame::Spare(i, _) => i,
                                Frame::Open(i, _) => i,
                                Frame::Tenth(Some(i), _, _) => i,
                                _ => {
                                    return Err("Error encountered on 3rd strike ball.");
                                }
                            }
                        }
                        Frame::Spare(i, j) => i + j,
                        Frame::Open(i, j) => i + j,
                        Frame::Tenth(Some(i), Some(j), _) => i + j,
                        _ => {
                            return Err("Error encountered on 2nd strike ball.");
                        }
                    }
                }
                Frame::Spare(i, j) => {
                    i + j +
                    match self.rolls[ind + 1] {
                        Frame::Strike => 10,
                        Frame::Spare(k, _) => k,
                        Frame::Open(k, _) => k,
                        Frame::Tenth(Some(k), _, _) => k,
                        _ => {
                            return Err("Error encountered on 2nd spare ball.");
                        }
                    }
                }
                Frame::Open(i, j) => i + j,
                Frame::Tenth(Some(i), Some(j), Some(k)) => i + j + k,
                Frame::Tenth(Some(i), Some(j), None) => i + j,
                _ => {
                    return Err("Ill-formed frame.");
                }
            }
        }

        Ok(result)
    }
}