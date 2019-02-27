use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Piece {
    Empty,
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    fn valid(&self) -> bool {
        let valid_x = self.x <= 7;
        let valid_y = self.y <= 7;

        valid_x && valid_y
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Move {
    pub player: Player,
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn valid(&self) -> bool {
        let from_valid = self.from.valid();
        let to_valid = self.to.valid();

        let diff_x = i32::from(self.to.x) - i32::from(self.from.x);
        let diff_y = i32::from(self.to.y) - i32::from(self.from.y);

        let mut valid_move = true;

        // TODO: write simpler check for horse move
        valid_move &= (diff_x.abs()).min(diff_y.abs()) == 1;
        valid_move &= (diff_x - diff_y).abs() == 1;

        from_valid && to_valid && valid_move
    }
}

pub type History = Vec<Move>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Board(pub [[Piece; 8]; 8]);

impl Default for Board {
    fn default() -> Board {
        use Piece::*;
        Board([
            [Empty, Black, Empty, Black, Empty, Black, Empty, Black],
            [Black, Empty, Black, Empty, Black, Empty, Black, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [White, Empty, White, Empty, White, Empty, White, Empty],
            [Empty, White, Empty, White, Empty, White, Empty, White],
        ])
    }
}
