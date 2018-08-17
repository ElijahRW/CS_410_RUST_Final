/*
*/
//MOUSE DEBUG INFO: Referenced From: https://github.com/PistonDevelopers/piston-examples/blob/master/user_input/src/main.rs

extern crate find_folder;
extern crate piston_window;

extern crate serde;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

mod piston_translator;
mod pong_ball;
mod ui_parser;
//This is our game module (Pong demonstration.)
mod application;
mod pong_paddle;


use application::Application;


fn main() {

    let mut game = Application::new_app_default_path();
    game.run();
}


