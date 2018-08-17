/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission.
*/

extern crate find_folder;
extern crate piston_window;
extern crate serde;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

mod piston_translator;
mod pong_ball;
mod ui_parser;
//This is our game module (Pong game demonstration.)
mod application;
mod pong_paddle;

use application::Application;

//Simple game logic is run from here
fn main() {
    let mut game = Application::new_app_default_path();
    game.run();
}
