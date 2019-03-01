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
        let valid_from = self.from.valid();
        let valid_to = self.to.valid();

        let diff_x = i32::from(self.to.x) - i32::from(self.from.x);
        let diff_y = i32::from(self.to.y) - i32::from(self.from.y);

        let mut valid_move = true;

        // TODO: write simpler check for horse move
        valid_move &= (diff_x.abs()).min(diff_y.abs()) == 1;
        valid_move &= (diff_x.abs()).max(diff_y.abs()) == 2;

        valid_from && valid_to && valid_move
    }
}

pub type History = Vec<Move>;

pub type Board = [[Piece; 8]; 8];
