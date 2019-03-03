use crate::types::*;
use crate::ui::Ui;
use crate::ui::UiMessage;
use serde::Serialize;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub struct Controller {
    pub rx: mpsc::Receiver<ControllerMessage>,
    pub tx: mpsc::Sender<ControllerMessage>,
    pub socket: TcpStream,
    pub ui: Ui,
}

pub enum ControllerMessage {
    /// Used to send updated data to server, if move was performed.
    MovePerformed(Move),
    /// Used to send updated gamestate from server to ui.
    UpdateState(GameState),
    /// Used to notify ui, that server is down.
    ServerIsDown,
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
                tx: tx.clone(),
                socket: stream.unwrap(),
                ui: Ui::new(tx.clone()),
            })
        }
    }
    /// Write json data to server.
    fn write_json_data(&mut self, data: &impl Serialize) {
        let mut buffer = serde_json::to_vec(data).expect("Serialization errror");
        buffer.push(b'\n');

        let _ = self.socket.write_all(&buffer);
    }
    // /// Get new state from server
    // fn get_new_state(&mut self) -> GameState {
    //     state
    // }

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
        let controller_tx = self.tx.clone();
        let socket = self.socket.try_clone();
        if socket.is_err() {
            panic!("Can't clone server socket.")
        }
        let socket = socket.unwrap();
        let _client = thread::spawn(move || {
            let mut buffer = String::new();
            let mut reader = BufReader::new(socket);
            loop {
                let line_res = reader.read_line(&mut buffer);
                if line_res.is_ok() {
                    if buffer.is_empty() {
                        controller_tx.send(ControllerMessage::ServerIsDown).unwrap();
                        break;
                    } else {
                        let state: GameState = serde_json::from_str(&buffer).unwrap();
                        controller_tx
                            .send(ControllerMessage::UpdateState(state.clone()))
                            .unwrap();
                        if state.finished {
                            break;
                        }
                    }
                } else {
                    controller_tx.send(ControllerMessage::ServerIsDown).unwrap();
                    break;
                }
                buffer.clear();
            }
        });
        while self.ui.step() {
            while let Some(message) = self.rx.try_iter().next() {
                // Handle messages arriving from the UI.
                match message {
                    ControllerMessage::MovePerformed(new_state) => {
                        self.write_json_data(&new_state);
                        // let server_state = self.get_new_state();
                    }
                    ControllerMessage::UpdateState(server_state) => {
                        self.ui
                            .ui_tx
                            .send(UiMessage::UpdateState(server_state))
                            .expect("Can't update board");
                    }
                    ControllerMessage::ServerIsDown => self
                        .ui
                        .ui_tx
                        .send(UiMessage::ServerIsDown)
                        .expect("Can't update server state."),
                };
            }
        }
    }
}
