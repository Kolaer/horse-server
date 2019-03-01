mod types;
use types::*;

extern crate serde;

use cursive::direction::Direction;
use cursive::event::Key;
use cursive::event::{Event, EventResult, MouseEvent};
use cursive::view::Identifiable;
use cursive::views::{BoxView, Button, Dialog, LinearLayout, ListView, ScrollView, TextView};
use cursive::Cursive;
use cursive::Printer;
use cursive::Vec2;
use serde::Serialize;
use std::fmt::Display;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

/// Send object through socket.
fn write_json_data(stream: &mut TcpStream, data: &impl Serialize) {
    let mut buffer = serde_json::to_vec(data).expect("Serialization errror");
    buffer.push(b'\n');

    stream
        .write_all(&buffer)
        .expect("Error while sending data to client");
}

/// This function will open connection and update tui.
fn start_game(siv: &mut Cursive) {
    let mut stream = TcpStream::connect("127.0.0.1:31337").expect("Can't connect server.");
    let mut board_view = BoardView::default();
    let mut buffer = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut buffer).unwrap();
    let current_player = match buffer.as_ref() {
        "White" => Some(Player::White),
        "Black" => Some(Player::Black),
        _ => None,
    };
    println!("{:?}", &current_player);
    buffer.clear();
    reader.read_line(&mut buffer).unwrap();
    let mut game_state: GameState = serde_json::from_str(&buffer).unwrap();
    board_view.gamestate = game_state;
    board_view.player = current_player;
    siv.pop_layer();

    let screen_size = siv.screen_size();
    let right_panel = LinearLayout::vertical()
        .child(TextView::new(format!(
            "Your color: {:#?}",
            board_view.player
        )))
        .child(
            TextView::new(format!(
                "Current turn: {:#?}",
                &board_view.gamestate.current_player
            ))
            .with_id("current_turn"),
        )
        .child(TextView::new("<h> for help."))
        .child(
            Dialog::new()
                .title("History")
                .content(ScrollView::new(ListView::new().with_id("history"))),
        );

    let board_layout = BoxView::with_fixed_size(
        (screen_size.x / 2, screen_size.y),
        board_view.with_id("board"),
    );
    let history_layout = BoxView::with_fixed_size((screen_size.x / 2, screen_size.y), right_panel);
    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(Dialog::new().title("Game").content(board_layout))
            .child(Dialog::new().title("Info").content(history_layout)),
    );
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

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('h', show_help);
    siv.add_layer(
        Dialog::new().title("Horses").content(
            LinearLayout::vertical()
                .child(TextView::new(
                    "Welcome to Horses\nWould you like to start game?",
                ))
                .child(
                    LinearLayout::horizontal()
                        .child(Button::new("Yes", move |s| start_game(s)))
                        .child(Button::new("No", |s| s.quit())),
                ),
        ),
    );
    // start_game(&mut siv);
    siv.run()
}
