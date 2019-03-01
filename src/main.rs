extern crate serde;
#[macro_use]
extern crate crossbeam;

mod gamesate;
mod types;

use crossbeam::channel::{bounded, Sender};
use gamesate::GameState;
use serde::Serialize;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use types::*;


/// Writes serializable data to TcpStream using \n as separator.
fn write_json_data(stream: &mut TcpStream, data: &impl Serialize) {
    let mut buffer = serde_json::to_vec(data).expect("Serialization errror");
    buffer.push(b'\n');

    stream
        .write_all(&buffer)
        .expect("Error while sending data to client");
}

/// Handles data from player: parses moves from TcpStream and sending them to channel.
fn handle_players_moves(
    player: Player,
    stream: Arc<RwLock<TcpStream>>,
    updates_chan: Sender<Option<Move>>,
) {
    let stream = stream.clone();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        {
            let stream = stream.read().unwrap();
            let mut reader = BufReader::new(&(*stream));

            reader
                .read_line(&mut buffer)
                .expect("Error while getting data from client");
            buffer.trim();

            let mv: Move = serde_json::from_str(&buffer).expect("Deserialization error");

            let msg = if mv.player == player { Some(mv) } else { None };

            updates_chan.send(msg).unwrap();
        }
    }
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:31337".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");
    println!("Listening on {}", addr);

    // Vector of players' tcp connections.
    let connections = Arc::new(RwLock::new(vec![]));
    let game_state = Arc::new(RwLock::new(GameState::default()));

    let (white_moves_writer, white_moves_reader) = bounded(1);
    let (black_moves_writer, black_moves_reader) = bounded(1);

    let conns = connections.clone();
    let gm_state = game_state.clone();

    // Main server, which accepts incomming tcp connections & runs some basic logic.
    // Runs in another thread.
    let _server = thread::spawn(move || {
        listener.incoming().for_each(|stream| {
            if stream.is_err() {
                panic!("Accept error: {:#?}", stream);
            }

            let mut stream = stream.unwrap();

            {
                let game_state = gm_state.read().unwrap();
                let mut conns = conns.write().unwrap();
                let player_count = conns.len();

                let player = match player_count {
                    0 => Some(Player::White),
                    1 => Some(Player::Black),
                    _ => None,
                };

                // Sending player & game state info to new player
                write_json_data(&mut stream, &player);
                write_json_data(&mut stream, &(*game_state));

                let stream = Arc::new(RwLock::new(stream));

                // Starting player handler in a new thread
                if let Some(player) = player {
                    let stream_clone = stream.clone();
                    let chan = match player {
                        Player::White => white_moves_writer.clone(),
                        Player::Black => black_moves_writer.clone(),
                    };
                    thread::spawn(move || {
                        handle_players_moves(Player::Black, stream_clone, chan);
                    });
                }

                // Saving tcp stream for future update sending
                conns.push(stream);
            }
        });
    });

    let game_state = game_state.clone();
    let connections = connections.clone();

    // Main game logic:
    // 1) get moves from players
    // 2) try to apply them
    // 3) send updates to all players
    loop {
        {
            let game_state = game_state.read().unwrap();

            if game_state.is_finished() {
                println!("Winner is {:?}", game_state.get_winner().unwrap());
                return;
            }
        }

        {
            let mut game_state = game_state.write().unwrap();
            let connections = connections.read().unwrap();

            let apply_message = |msg, game_state: &mut GameState| {
                if let Ok(Some(mv)) = msg {
                    let new_game_state = (*game_state).make_move(mv);

                    if let Some(new_game_state) = new_game_state {
                        *game_state = new_game_state;
                    }
                }
            };

            // read move & maybe apply it
            select! {
                recv(white_moves_reader) -> msg => apply_message(msg, &mut game_state),
                recv(black_moves_reader) -> msg => apply_message(msg, &mut game_state),
            }

            // send updates
            for stream in (*connections).iter() {
                let stream = stream.clone();
                let mut stream = stream.write().unwrap();
                write_json_data(&mut stream, &(*game_state));
            }
        }
    }
}
