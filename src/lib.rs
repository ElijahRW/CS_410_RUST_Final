/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission
*/
extern crate find_folder;
extern crate piston_window;
extern crate serde;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

pub mod piston_translator;
pub mod ui_parser;
mod application;
pub mod pong_paddle;
pub mod pong_ball;
