use serde::Deserialize;
use serde::Serialize;

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

impl PartialEq<Piece> for Player {
    fn eq(&self, piece: &Piece) -> bool {
        match (self, piece) {
            (Player::White, Piece::White) => true,
            (Player::Black, Piece::Black) => true,
            _ => false,
        }
    }
}

impl PartialEq<Player> for Piece {
    fn eq(&self, player: &Player) -> bool {
        player == self
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Move {
    pub player: Player,
    pub from: Position,
    pub to: Position,
}

pub type History = Vec<Move>;

pub type Board = [[Piece; 8]; 8];
