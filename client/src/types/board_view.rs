use crate::controller::ControllerMessage;
use crate::types::Move;
use crate::types::Position;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseEvent};
use std::sync::mpsc;
// use cursive::event::{Event, EventResult, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::Printer;
use cursive::Vec2;

use crate::types::GameState;
use crate::types::Piece;
use crate::types::Player;

pub struct BoardView {
    /// Current game state.
    pub gamestate: GameState,
    /// All available positions to move onto.
    pub available: Vec<Vec2>,
    /// Chess position player focusing on.
    pub focused: Option<Vec2>,
    /// Current player.
    pub player: Option<Player>,
    /// Channel to send messages to contoroller.
    controller_tx: mpsc::Sender<ControllerMessage>,
}

// impl Default for BoardView {
//     fn default() -> Self {
//         let gamestate = GameState::default();
//         let available = Vec::new();
//         let focused = None;
//         let player = None;
//         BoardView {
//             gamestate,
//             available,
//             focused,
//             player,
//         }
//     }
// }

impl BoardView {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let gamestate = GameState::default();
        let available = Vec::new();
        let focused = None;
        let player = None;
        BoardView {
            gamestate,
            available,
            focused,
            player,
            controller_tx,
        }
    }

    /// Map coordinates from View to chessboard coordinate
    fn get_cell(&mut self, position: Vec2, offset: Vec2) -> Option<Vec2> {
        if let Some(pos) = position.checked_sub(offset) {
            let pos = Vec2::from((pos.x / 4, pos.y / 4));
            if pos.fits_in((7, 7)) {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Add all available positions to move from focused into
    /// BoardView.available.
    fn update_available(&mut self) {
        fn check_cell(focused: Vec2, x: i8, y: i8, available: &mut Vec<Vec2>) {
            let x = focused.x as i8 + x;
            let y = focused.y as i8 + y;
            if x > 7 || x < 0 {
                return;
            }
            if y > 7 || y < 0 {
                return;
            }
            // return Some(Vec2::from((x as usize, y as usize)));
            available.push(Vec2::from((x as usize, y as usize)));
        }
        if let Some(cell) = self.focused {
            check_cell(cell.clone(), 2, 1, &mut self.available);
            check_cell(cell.clone(), 2, -1, &mut self.available);
            check_cell(cell.clone(), -2, 1, &mut self.available);
            check_cell(cell.clone(), -2, -1, &mut self.available);
            check_cell(cell.clone(), 1, 2, &mut self.available);
            check_cell(cell.clone(), -1, 2, &mut self.available);
            check_cell(cell.clone(), 1, -2, &mut self.available);
            check_cell(cell.clone(), -1, -2, &mut self.available);
            // self.available.push(cell);
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        let letters = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        for ci in 1..5 {
            for cj in 1..5 {
                for (i, row) in self.gamestate.board.iter().enumerate() {
                    let i_size = i * 4 + ci;
                    for (j, piece) in row.iter().enumerate() {
                        let j_size = j * 4 + cj;
                        let is_black = (i + j) % 2 == 0;
                        let mut back_color = match is_black {
                            true => Color::RgbLowRes(3, 3, 3),
                            false => Color::Dark(BaseColor::White),
                        };
                        let position = Vec2::from((j, i));
                        let available = self.available.iter().any(|el| el == &position);
                        if available {
                            back_color = Color::RgbLowRes(1, 3, 1);
                        }
                        let print_text = (ci == 3 && cj == 3)
                            || (ci == 2 && cj == 2)
                            || (ci == 2 && cj == 3)
                            || (ci == 3 && cj == 2);
                        let text = match piece {
                            Piece::Empty => " ",
                            Piece::Black => {
                                if print_text {
                                    "♞"
                                } else {
                                    " "
                                }
                            }
                            Piece::White => {
                                if print_text {
                                    "♘"
                                } else {
                                    " "
                                }
                            }
                        };

                        printer.with_color(
                            ColorStyle::new(Color::Dark(BaseColor::Black), back_color),
                            |printer| printer.print((j_size, i_size), text),
                        );

                        let mut letter = " ";
                        if cj == 2 {
                            letter = letters[j];
                        }

                        printer.with_color(
                            ColorStyle::new(
                                Color::Dark(BaseColor::Black),
                                Color::Dark(BaseColor::White),
                            ),
                            |printer| printer.print((j_size, 0), letter),
                        );
                    }
                    if ci == 2 {
                        printer.with_color(
                            ColorStyle::new(
                                Color::Dark(BaseColor::Black),
                                Color::Dark(BaseColor::White),
                            ),
                            |printer| printer.print((0, i_size), &i.to_string()),
                        );
                    } else {
                        printer.with_color(
                            ColorStyle::new(
                                Color::Dark(BaseColor::Black),
                                Color::Dark(BaseColor::White),
                            ),
                            |printer| printer.print((0, i_size), " "),
                        );
                    }
                }
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::from((33, 33))
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_btn),
            } => {
                if let Some(cell) = self.get_cell(position, offset) {
                    let test_color = match (&self.gamestate.board[cell.y][cell.x], &self.player) {
                        (Piece::Black, Some(Player::Black)) => true,
                        (Piece::White, Some(Player::White)) => true,
                        _ => false,
                    };
                    if test_color {
                        self.focused = Some(cell);
                        self.update_available();
                        return EventResult::Consumed(None);
                    }
                }
            }
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(_),
            } => {
                if let Some(pos) = self.get_cell(position, offset) {
                    if let Some(player) = &self.player {
                        if let Some(focused) = self.focused {
                            let chess = match player.clone() {
                                Player::White => Piece::White,
                                Player::Black => Piece::Black,
                            };
                            let available = self.available.iter().any(|el| el == &pos);
                            if available {
                                self.focused = None;
                                self.gamestate.board[focused.y][focused.x] = Piece::Empty;
                                self.gamestate.board[pos.y][pos.x] = chess;
                                self.available.clear();
                                let chess_move = Move {
                                    player: player.clone(),
                                    from: Position {
                                        x: focused.x as u8,
                                        y: focused.y as u8,
                                    },
                                    to: Position {
                                        x: pos.x as u8,
                                        y: pos.y as u8,
                                    },
                                };
                                self.controller_tx
                                    .send(ControllerMessage::MovePerformed(chess_move))
                                    .unwrap();
                            }
                        }
                    }
                }
            }
            _ => (),
        }
        EventResult::Ignored
    }
}
