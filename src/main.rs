/*
***Copied Code from Piston examples
***Current purpose is to provide an example window to demonstrate UI load
***comment will be revised once this source file has been adequately
***changed to reflect original content
***For original source, see:
*/
//MOUSE DEBUG INFO: Referenced From: https://github.com/PistonDevelopers/piston-examples/blob/master/user_input/src/main.rs

extern crate find_folder;
extern crate piston_window;

//use piston_window::draw_state::Blend;

//use piston_window::*;

//extern crate ui_parser;
extern crate serde;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

mod piston_translator;
mod pong_ball;
mod ui_parser;
//This is our game module (Pong demonstration.)
mod application;


use application::*;
//use pong_ball::*;
//use piston_translator::*;
//use ui_parser::*;
//use pong_ball::*;


fn main() {

    let mut game = Application::new();
    game.run();
}

//

//fn get_ui_buttons(file_path: &str) -> Vec<ButtonData> {
//    ButtonData::read_from_file(file_path)
//}
