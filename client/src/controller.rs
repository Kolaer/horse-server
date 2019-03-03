use crate::types::*;
use crate::ui::Ui;
use crate::ui::UiMessage;
use serde::Serialize;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;

pub struct Controller {
    pub rx: mpsc::Receiver<ControllerMessage>,
    pub socket: TcpStream,
    pub ui: Ui,
}

pub enum ControllerMessage {
    /// Used to send updated data to server, if move was performed.
    MovePerformed(Move),
}

impl Controller {
    /// Create a new controller
    pub fn new() -> Result<Controller, String> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();
        let stream = TcpStream::connect("127.0.0.1:31337");
        if stream.is_err() {
            Err("Can't connect server.".to_string())
        } else {
            Ok(Controller {
                rx: rx,
                socket: stream.unwrap(),
                ui: Ui::new(tx.clone()),
            })
        }
    }

    fn write_json_data(&mut self, data: &impl Serialize) {
        let mut buffer = serde_json::to_vec(data).expect("Serialization errror");
        buffer.push(b'\n');

        let _ = self.socket.write_all(&buffer);
    }

    fn get_new_state(&mut self) -> GameState {
        let mut buffer = String::new();
        let mut reader = BufReader::new(&self.socket);
        reader.read_line(&mut buffer).unwrap();
        let state: GameState = serde_json::from_str(&buffer).unwrap();
        state
    }

    /// Run the controller
    pub fn run(&mut self) {
        let mut buffer = String::new();
        let mut reader = BufReader::new(&self.socket);
        reader.read_line(&mut buffer).unwrap();
        let current_player: Player = serde_json::from_str(&buffer).unwrap();
        self.ui
            .ui_tx
            .send(UiMessage::UpdateProfile(serde::export::Some(
                current_player,
            )))
            .unwrap();
        buffer.clear();
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                // Handle messages arriving from the UI.
                match message {
                    ControllerMessage::MovePerformed(new_state) => {
                        self.write_json_data(&new_state);
                        let server_state = self.get_new_state();
                        self.ui
                            .ui_tx
                            .send(UiMessage::UpdateState(server_state))
                            .expect("Can't update board");
                    }
                };
            }
        }
    }
}
