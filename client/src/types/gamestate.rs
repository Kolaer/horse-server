use crate::types::Board;
use crate::types::History;
use crate::types::Player;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub move_history: History,
    pub finished: bool,
    pub winner: Option<Player>,
}

impl Default for GameState {
    fn default() -> GameState {
        use crate::types::Piece::*;

        let board = [
            [Empty, Black, Empty, Black, Empty, Black, Empty, Black],
            [Black, Empty, Black, Empty, Black, Empty, Black, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
            [White, Empty, White, Empty, White, Empty, White, Empty],
            [Empty, White, Empty, White, Empty, White, Empty, White],
        ];
        GameState {
            board,
            current_player: Player::White,
            move_history: vec![],
            finished: false,
            winner: None,
        }
    }
}
