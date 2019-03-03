use crate::types::*;
use crate::controller::ControllerMessage;
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
}

pub enum UiMessage {
    UpdateProfile(Option<Player>),
    UpdateBoard(GameState),
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
        };

        // Create a view tree with a TextArea for input, and a
        // TextView for output.
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
                    }
                    self.cursive.call_on_id("board", |view: &mut BoardView| {
                        view.player = profile.clone()
                    });
                    self.cursive.call_on_id("profile", |view: &mut TextView| {
                        view.set_content(format!("Your color: {}", profile_type))
                    });
                }
                UiMessage::UpdateBoard(new_state) => {
                    self.cursive.call_on_id("board", |view: &mut BoardView| {
                        view.gamestate = new_state.clone();
                    });
                    self.cursive
                        .call_on_id("current_turn", |view: &mut TextView| {
                            view.set_content(format!("{:?}", new_state.clone().current_player))
                        });
                }
            }
        }

        // Step the UI
        self.cursive.step();

        true
    }
}

/// show help.
fn show_help(siv: &mut Cursive) {
    siv.add_layer(Dialog::info(
        "
<ESC>: close game.
<h>: show help.
'W': white horses.
'B': black horses",
    ));
}
