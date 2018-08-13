//Translator Module, This file currently must be ENGINE specific. It will rely on the syntax defined
// in parser.rs
// This translator should hopefully be as LIGHT as possible, since our current implementation requires
//that a translator module exists for every potential engine.
extern crate find_folder;
extern crate piston_window;


use ui_parser::*;
//extern crate ui_parser;

use piston_translator::piston_window::draw_state::Blend;

use piston_translator::piston_window::*;


struct RectangleData {
    dimentions: types::Rectangle,
    color: [f32; 4],
}

fn test () {
    let x = RectangleData::new();
}

impl RectangleData {
    pub fn new() -> Self {
        RectangleData{
            dimentions: rectangle::rectangle_by_corners(0.0,0.0,0.0,0.0),
            //dimentions: Rectangle::new(button.color), //rectangle::rectangle_by_corners(0.0,0.0,0.0,0.0),
            color: [0.0,0.0,0.0,0.0],
            //rectangle::ne
        }
    }

    pub fn readFromFile(filePath: String) -> Option<Self> {

        None
    }
}

/*
    The Goal is to implement an iterator of items (Can't be excusivly of one type that the main logic loop may rapidly iterate through and add to the draw logic without significant loss of performance
*/