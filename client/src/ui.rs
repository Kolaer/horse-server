use crate::controller::ControllerMessage;
use crate::types::*;
use cursive::event::Key;
use cursive::view::*;
use cursive::views::*;
use cursive::Cursive;
use std::sync::mpsc;

pub struct Ui {
    pub cursive: Cursive,
    pub ui_rx: mpsc::Receiver<UiMessage>,
    pub ui_tx: mpsc::Sender<UiMessage>,
    pub controller_tx: mpsc::Sender<ControllerMessage>,
    pub player: Option<Player>,
}

pub enum UiMessage {
    UpdateProfile(Option<Player>),
    UpdateState(GameState),
    ServerIsDown,
}

impl Ui {
    /// Create a new Ui object.  The provided `mpsc` sender will be used
    /// by the UI to send messages to the controller.
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Ui {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();
        let mut ui = Ui {
            cursive: Cursive::default(),
            ui_tx: ui_tx,
            ui_rx: ui_rx,
            controller_tx: controller_tx.clone(),
            player: None,
        };

        ui.cursive.set_fps(30);
        ui.cursive.add_global_callback(Key::Esc, move |c| c.quit());
        ui.cursive.add_global_callback('h', move |c| show_help(c));
        let gamestate = BoardView::new(controller_tx.clone());

        let screen_size = ui.cursive.screen_size();
        let right_panel = LinearLayout::vertical()
            .child(TextView::new("Your color: None").with_id("profile"))
            .child(TextView::new("Current turn: None").with_id("current_turn"))
            .child(TextView::new("<h> for help."))
            .child(
                Dialog::new()
                    .title("History")
                    .content(ScrollView::new(ListView::new().with_id("history"))),
            );

        let board_layout = BoxView::with_fixed_size(
            (screen_size.x / 2, screen_size.y),
            gamestate.with_id("board"),
        );
        let history_layout =
            BoxView::with_fixed_size((screen_size.x / 2, screen_size.y), right_panel);
        ui.cursive.add_fullscreen_layer(
            LinearLayout::horizontal()
                .child(Dialog::new().title("Game").content(board_layout))
                .child(Dialog::new().title("Info").content(history_layout)),
        );
        ui
    }

    /// Step the UI by calling into Cursive's step function, then
    /// processing any UI messages.
    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        // Process any pending UI messages
        while let Some(message) = self.ui_rx.try_iter().next() {
            match message {
                UiMessage::UpdateProfile(profile) => {
                    let mut profile_type = "spectator";
                    if let Some(player) = profile.clone() {
                        match player {
                            Player::White => profile_type = "White",
                            Player::Black => profile_type = "Black",
                        }
                        self.player = Some(player.clone());
                    }
                    self.cursive.call_on_id("board", |view: &mut BoardView| {
                        view.player = profile.clone()
                    });
                    self.cursive.call_on_id("profile", |view: &mut TextView| {
                        view.set_content(format!("Your color: {}", profile_type))
                    });
                }
                UiMessage::UpdateState(new_state) => {
                    self.cursive.call_on_id("board", |view: &mut BoardView| {
                        view.gamestate = new_state.clone();
                    });
                    self.cursive
                        .call_on_id("current_turn", |view: &mut TextView| {
                            view.set_content(format!(
                                "Current turn: {:?}",
                                new_state.clone().current_player
                            ))
                        });
                    self.cursive.call_on_id("history", |view: &mut ListView| {
                        view.clear();
                        let letters = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
                        for horse_move in new_state.clone().move_history {
                            let text = format!(
                                "{:?} {}{} -> {}{}",
                                horse_move.player,
                                letters[horse_move.from.x as usize],
                                horse_move.from.y,
                                letters[horse_move.to.x as usize],
                                horse_move.from.x
                            );
                            view.add_child(" ", TextView::new(text));
                        }
                    });
                    if new_state.finished {
                        let mut message = String::from("Opponent was disconnected.");
                        let mut button_msg = String::from("Ok");
                        if let Some(winner) = new_state.winner {
                            if None == self.player {
                                message = format!("{:?} won.", winner);
                            } else if Some(winner) == self.player {
                                message = String::from("You won!");
                                button_msg = String::from("Yay!");
                            } else {
                                message = String::from("You lose.");
                                button_msg = String::from("Ah");
                            }
                        }
                        self.cursive.add_layer(
                            Dialog::new().content(
                                LinearLayout::vertical()
                                    .child(TextView::new(message))
                                    .child(
                                        LinearLayout::horizontal()
                                            .child(Button::new(button_msg, |s| s.quit())),
                                    ),
                            ),
                        );
                    }
                }
                UiMessage::ServerIsDown => {
                    self.cursive.add_layer(
                        Dialog::new().content(
                            LinearLayout::vertical()
                                .child(TextView::new(
                                    "Uh, oh. Seems like the server is down\nor other player left.",
                                ))
                                .child(
                                    LinearLayout::horizontal()
                                        .child(Button::new("Oh.", |s| s.quit())),
                                ),
                        ),
                    );
                }
            }
        }

        // Step the UI
        self.cursive.step();

        true
    }
}

/// show help dialog.
fn show_help(siv: &mut Cursive) {
    siv.add_layer(Dialog::info(
        "
<ESC>: close game.
<h>: show help.
'W': white horses.
'B': black horses",
    ));
}
