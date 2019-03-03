use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GameState {
    board: Board,
    current_player: Player,
    move_history: History,
    finished: bool,
    winner: Option<Player>,
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

impl GameState {
    pub fn make_move(&mut self, mv: Move) {
        if mv.player != self.current_player {
            return;
        }

        if !mv.valid() {
            return;
        }

        let from_x = mv.from.x as usize;
        let from_y = mv.from.y as usize;

        let from_piece = self.board[from_y][from_x].clone();

        if from_piece == Piece::Empty {
            return;
        }

        let to_x = mv.to.x as usize;
        let to_y = mv.to.y as usize;

        let to_piece = self.board[to_y][to_x].clone();

        if from_piece == to_piece {
            return;
        }

        self.board[from_y][from_x] = Piece::Empty;
        self.board[to_y][to_x] = from_piece;

        self.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        self.move_history.push(mv.clone());

        self.set_winner();
    }

    fn count_pieces(&self) -> (u8, u8) {
        let mut count_white = 0;
        let mut count_black = 0;

        for row in &self.board {
            for piece in row {
                match piece {
                    Piece::White => count_white += 1,
                    Piece::Black => count_black += 1,
                    _ => {}
                }
            }
        }

        (count_white, count_black)
    }

    fn set_winner(&mut self) {
        if self.finished {
            return;
        }

        let (count_white, count_black) = self.count_pieces();

        self.finished = (count_black < 4) || (count_white < 4);
        self.winner = {
            if count_black < 4 {
                Some(Player::White)
            } else if count_white < 4 {
                Some(Player::Black)
            } else {
                None
            }
        };
    }

    pub fn get_winner(&self) -> Option<Player> {
        self.winner.clone()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
