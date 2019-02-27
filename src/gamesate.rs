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
        GameState {
            board: Board::default(),
            current_player: Player::White,
            move_history: vec![],
            finished: false,
            winner: None,
        }
    }
}

impl GameState {
    // TODO: maybe switch to mutating same game state?
    fn make_move(&self, mv: Move) -> Option<GameState> {
        if mv.player != self.current_player {
            return None;
        }

        if !mv.valid() {
            return None;
        }

        let from_x = mv.from.x as usize;
        let from_y = mv.from.y as usize;

        let from_piece = &self.board.0[from_y][from_x];

        if *from_piece == Piece::Empty {
            return None;
        }

        let to_x = mv.to.x as usize;
        let to_y = mv.to.y as usize;

        let to_piece = &self.board.0[to_y][to_x];

        if *from_piece == *to_piece {
            return None;
        }

        let mut new_game_state = self.clone();

        (new_game_state.board.0)[from_x][from_y] = Piece::Empty;
        (new_game_state.board.0)[to_x][to_y] = (*from_piece).clone();

        new_game_state.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        new_game_state.move_history.push(mv.clone());

        new_game_state.set_winner();

        Some(new_game_state)
    }

    fn count_pieces(&self) -> (u8, u8) {
        let mut count_white = 0;
        let mut count_black = 0;

        for row in &self.board.0 {
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
}
