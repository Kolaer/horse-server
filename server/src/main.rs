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
use std::process;
use std::sync::{Arc, RwLock};
use std::thread;
use types::*;

/// Writes serializable data to TcpStream using \n as separator.
fn write_json_data(stream: &mut TcpStream, data: &impl Serialize) {
    let mut buffer = serde_json::to_vec(data).expect("Serialization errror");
    buffer.push(b'\n');

    let _ = stream.write_all(&buffer);
}

enum ChannelMsg<T> {
    Close,
    Msg(T),
}

/// Handles data from player: parses moves from TcpStream and sending them to channel.
fn handle_players_moves(
    player: Player,
    stream: TcpStream,
    updates_chan: Sender<ChannelMsg<Option<Move>>>,
) {
    let mut buffer = String::new();
    let mut reader = BufReader::new(&stream);

    loop {
        buffer.clear();
        reader
            .read_line(&mut buffer)
            .expect("Error while getting data from client");
        let buffer = buffer.trim();

        if buffer.is_empty() {
            updates_chan.send(ChannelMsg::Close).unwrap();
            return;
        }

        let msg: Result<Move, _> = serde_json::from_str(&buffer);

        let msg = match msg {
            Ok(msg) => {
                if msg.player == player {
                    Some(msg)
                } else {
                    None
                }
            }
            _ => None,
        };

        let msg = ChannelMsg::Msg(msg);

        updates_chan.send(msg).unwrap();
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

                // Starting player handler in a new thread
                if let Some(player) = player {
                    let stream_clone = stream.try_clone().expect("IO error: clonning TcpStream");
                    let chan = match player {
                        Player::White => white_moves_writer.clone(),
                        Player::Black => black_moves_writer.clone(),
                    };
                    thread::spawn(move || {
                        handle_players_moves(player, stream_clone, chan);
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
            let apply_message = |msg: Result<ChannelMsg<Option<Move>>, crossbeam::RecvError>,
                                 game_state: &mut GameState| {
                if msg.is_err() {
                    return;
                }
                let msg = msg.unwrap();

                match msg {
                    ChannelMsg::Close => {
                        eprintln!("Some player left the game");
                        process::exit(1);
                    }
                    ChannelMsg::Msg(msg) => {
                        if let Some(mv) = msg {
                            (*game_state).make_move(mv);
                        }
                    }
                }
            };

            // read move & maybe apply it
            select! {
                recv(white_moves_reader) -> msg => {
                    let mut game_state = game_state.write().unwrap();
                    apply_message(msg, &mut game_state)
                },
                recv(black_moves_reader) -> msg => {
                    let mut game_state = game_state.write().unwrap();
                    apply_message(msg, &mut game_state)
                },
            }

            {
                let connections = connections.read().unwrap();
                // send updates
                for stream in (*connections).iter() {
                    let mut stream = stream.try_clone().expect("IO error: cloning TcpStream ");
                    write_json_data(&mut stream, &(*game_state));
                }
            }
        }
    }
}
