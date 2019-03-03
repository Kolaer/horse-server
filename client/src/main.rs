extern crate serde;

mod types;

mod controller;
use controller::Controller;

mod ui;

fn main() {
    let controller = Controller::new("127.0.0.1:31337");
    match controller {
        Ok(mut controller) => controller.run(),
        Err(e) => println!("Error: {}", e),
    };
}
