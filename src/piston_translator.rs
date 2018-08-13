//Translator Module, This file currently must be ENGINE specific. It will rely on the syntax defined
// in parser.rs
// This translator should hopefully be as LIGHT as possible, since our current implementation requires
//that a translator module exists for every potential engine.
extern crate find_folder;
extern crate piston_window;

use ui_parser::*;
//extern crate ui_parser;

//use piston_translator::piston_window::draw_state::Blend;

use piston_translator::piston_window::*;

pub struct ButtonData {
    pub dimensions: types::Rectangle,
    pub position_x: f64,
    pub position_y: f64,
    pub color: [f32; 4],
}


#[test]
fn test_basic_button_translation () {
    //Todo: add manual Button Creation
    let button = UiButton::read("assets/GUI/example_button.xml");
    let x = ButtonData::new(button.unwrap());
}

impl ButtonData {
    pub fn new(button: UiButton) -> Self {
        ButtonData {
            dimensions: rectangle::rectangle_by_corners(
                0.0,
                0.0,
                button.dimensions.width as f64,
                button.dimensions.height as f64,
            ),
            position_x: Self::scale_for_f64(button.location.x),
            position_y: Self::scale_for_f64(button.location.y),
            color: [Self::scale_for_f32(button.color.r), Self::scale_for_f32(button.color.g), Self::scale_for_f32(button.color.b), Self::scale_for_f32(button.color.a)],
        }
        //result.dimensions.set();
    }
    fn scale_for_f32(x: u64) -> f32 {
        (x as f32) / 100.0
    }
    fn scale_for_f64(x: i32) -> f64 {
        x as f64 / 100.0
    }

    pub fn read_buttons_from_file(file_path: &str) -> Vec<ButtonData> {
        let buttons = Buttons::read(file_path);
        let mut result = Vec::new();
        let button_vector = buttons.unwrap(); //TODO: add correct match case.

        for button in button_vector.buttons {
            result.push(Self::new(button));
        }
        result
    }
}

/*
    The Goal is to implement an iterator of items (Can't be exclusively of one type that the main logic loop may rapidly iterate through and add to the draw logic without significant loss of performance
*/
