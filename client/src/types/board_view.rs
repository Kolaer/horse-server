use cursive::direction::Direction;
// use cursive::event::{Event, EventResult, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::Printer;
use cursive::Vec2;

use crate::types::GameState;
use crate::types::Piece;
use crate::types::Player;

#[derive(Debug)]
pub struct BoardView {
    pub gamestate: GameState,
    pub available: Vec<Vec<Vec2>>,
    pub focused: Option<Vec2>,
    pub player: Option<Player>,
}

impl Default for BoardView {
    fn default() -> Self {
        let gamestate = GameState::default();
        let available = Vec::new();
        let focused = None;
        let player = None;
        BoardView {
            gamestate,
            available,
            focused,
            player,
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
                        let back_color = match is_black {
                            true => Color::RgbLowRes(3, 3, 3),
                            false => Color::Dark(BaseColor::White),
                        };

                        // let color = match piece {
                        //     Piece::Black => Color::RgbLowRes(1, 3, 1),
                        //     Piece::White => Color::RgbLowRes(1, 3, 1),
                        //     Piece::Empty => Color::RgbLowRes(1, 3, 1),
                        //     _ => back_color,
                        // };

                        let text = match piece {
                            Piece::Empty => " ",
                            Piece::Black => "B",
                            Piece::White => "W",
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

    // fn on_event(&mut self, event: Event) -> EventResult {
    //     match event {
    //         Event::Mouse {
    //             offset,
    //             position,
    //             event: MouseEvent::Press(_btn),
    //         } => {
    //             if let Some(cell) = self.get_cell(position, offset) {
    //                 if self.gamesate.board[cell.y][cell.x] == self.gamestate.player {
    //                     self.focused = Some(cell);
    //                     self.show_moves();
    //                     return EventResult::Consumed(None);
    //                 }
    //             }
    //         }
    //         Event::Mouse {
    //             offset,
    //             position,
    //             event: MouseEvent::Release(_),
    //         } => {
    //             if let Some(pos) = self.get_cell(position, offset) {
    //                 if let Some(focused) = self.focused {
    //                     let chess = match self.current_player {
    //                         Player::White => Chess::White(false),
    //                         Player::Black => Chess::Black(false),
    //                     };
    //                     self.focused = None;
    //                     self.board[focused.y][focused.x] = Chess::Empty(false);
    //                     self.board[pos.y][pos.x] = chess;
    //                     self.hide_moves();
    //                 }
    //             }
    //         }
    //         _ => (),
    //     }
    //     EventResult::Ignored
    // }
}
