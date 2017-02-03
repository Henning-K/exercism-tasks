pub struct ChessPosition {
    x: i32,
    y: i32,
}

pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Result<Self, String> {
        if x < 0 || y < 0 || x > 7 || y > 7 {
            return Err("Invalid Position".to_string());
        }
        Ok(ChessPosition { x: x, y: y })
    }
}

impl Queen {
    pub fn new(cp: ChessPosition) -> Self {
        Queen { pos: cp }
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        self.pos.x == other.pos.x || self.pos.y == other.pos.y ||
        ((self.pos.x - other.pos.x) / (self.pos.y - other.pos.y)).abs() == 1
    }
}
