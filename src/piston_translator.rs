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


pub struct ButtonData {
    pub dimensions: types::Rectangle,
    pub position_x: u64,
    pub position_y: u64,
    pub color: [f32; 4],
}


/*fn test () {
    let x = ButtonData::new();
}*/

impl ButtonData {
    /*pub fn new() -> Self {
        ButtonData {
            dimentions: rectangle::rectangle_by_corners(0.0,0.0,0.0,0.0),
            position_x: 0,
            position_y: 0,
            //dimentions: Rectangle::new(button.color), //rectangle::rectangle_by_corners(0.0,0.0,0.0,0.0),
            color: [0.0,0.0,0.0,0.0],
            //rectangle::ne
        }
    }*/
    pub fn new(button: UiButton) -> Self {
        ButtonData {
            dimensions: rectangle::rectangle_by_corners(0.0, 0.0, button.dimensions.width as f64, button.dimensions.height as f64),
            position_x: 50,
            position_y: 50,
            color: [0.5,0.5,0.5,1.0],
        }
    }


    pub fn read_buttons_from_file(filePath: &str) -> Vec<ButtonData> {
        let buttons = Buttons::read(filePath);
        let mut result = Vec::new();
        let button_vector = buttons.unwrap();//TODO: add correct match case.

        for button in button_vector.buttons {
            result.push(Self::new(button));
        }
        result
    }
}

/*
    The Goal is to implement an iterator of items (Can't be exclusively of one type that the main logic loop may rapidly iterate through and add to the draw logic without significant loss of performance
*/