/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission
--File Description: This file is responsible for translating raw data parsed from the XML file (stored in ui_parser) and implementing gameready structures.
*/

//References: http://docs.piston.rs/piston_window/png/index.html
//http://docs.piston.rs/piston_window/image/index.html

//Translator Module, This file currently must be ENGINE specific. It will rely on the syntax defined in parser.rs

extern crate find_folder;
extern crate piston_window;
use self::piston_window::*;
use ui_parser::*;

//Button Data Represents a single object; (theoretically it could be a button/game_object/its supposed to be univeral in purpose)
//This structure will store the metric data parsed from the ui_parser objects
pub struct ButtonData {
    pub visible: bool,
    pub dimensions: types::Rectangle,
    pub position_x: f64,
    pub position_y: f64,
    pub color: [f32; 4],
    pub push_id: Option<String>,
}

/*
** UiObject is the core logic behind this project. All objects are parsed into this stuct and then placed
** into sepperate uses. This object may be used as a button, or a game object (such as a paddle).
** These objects are created dynamically by parsing an xml file using the RawButtonData structure defined in ui_parser.rs.
*/
impl UiObject for ButtonData {
    //Simple Collision Function (often Utilized to detect if a click occured within the button)
    fn is_inside(&self, [x, y]: [f64; 2]) -> bool {
        //Is Is between left and right borders
        if x >= self.position_x && (x <= (self.position_x + self.dimensions[2])) {
            //Is between top and bottom borders.
            if (y >= self.position_y) && (y <= self.position_y + self.dimensions[3]) {
                return true;
            }
        }
        false
    }
    //Draw function used to draw values in the Piston game window.
    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G)
    where
        G: Graphics,
    {
        if self.visible {
            Rectangle::new(self.color).draw(self.dimensions, &Default::default(), transform, g);
        }
    }
}

impl XmlButtonReadable for ButtonData {
    //This function is the heart of our translator. It reads in metric values from the
    // ui_parser structs and creates a relevant ui object
    fn new(button: UiButtonRaw) -> ButtonData {
        ButtonData {
            visible: match button.visible {
                Some(vis) => vis,
                None => false, //TODO: Serde currently doesn't properly match bool values. (Always returns true) INVESTIGATE
            },
            dimensions: rectangle::rectangle_by_corners(
                0.0,
                0.0,
                button.dimensions.width as f64,
                button.dimensions.height as f64,
            ),
            position_x: scale_for_f64(button.location.x),
            position_y: scale_for_f64(button.location.y),
            color: [
                scale_for_f32(button.color.r),
                scale_for_f32(button.color.g),
                scale_for_f32(button.color.b),
                scale_for_f32(button.color.a),
            ],
            push_id: button.push_id,
        }
    }

    //This function is the heart of our translator. It reads in semantic values from the
    // ui_parser structs and creates relevant ui objects
    fn new_with_screen_context(button: UiButtonRaw, size: &Size) -> Self {
        let style = button.location.style.clone();
        let mut value = Self::new(button);
        match style.as_ref() {
            "centered" => value.center_button(size),
            "right" => value.left_button(size),
            _ => (),
        };
        value
    }
    fn read_vec_from_file(file_path: &str) -> Vec<ButtonData> {
        let buttons = Buttons::read(file_path);
        let mut result = Vec::new();
        if let Some(button_vector) = buttons {
            for button in button_vector.buttons {
                result.push(Self::new(button));
            }
            result
        }
        else {
            Vec::new()
        }
    }

    fn read_from_file_w_context(file_path: &str, size: &Size) -> Vec<ButtonData> {
        let buttons = Buttons::read(file_path);
        let mut result = Vec::new();
        if let Some(button_vector) = buttons {
            for button in button_vector.buttons {
                result.push(Self::new_with_screen_context(button, size));
            }
            result
        } else {
            Vec::new()
        }
    }


    fn left_button(&mut self, size: &Size) {
        self.position_x = (size.width as f64) - self.position_y;
    }

    fn center_button(&mut self, size: &Size) {
        self.position_y = (size.height as f64) / 2.0;
        self.position_x = (size.width as f64) / 2.0;
    }
}

pub trait XmlButtonReadable: Sized {
    fn new(button: UiButtonRaw) -> Self;
    fn new_with_screen_context(button: UiButtonRaw, size: &Size) -> Self;
    fn read_vec_from_file(file_path: &str) -> Vec<Self>;
    fn read_from_file_w_context(file_path: &str, size: &Size) -> Vec<ButtonData>;
    fn left_button(&mut self, size: &Size);
    fn center_button(&mut self, size: &Size);
}
/*

*/
pub trait UiObject {
    //fn new(button: UiButtonRaw) -> Self;
    fn is_inside(&self, point: [f64; 2]) -> bool;
    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G)
    where
        G: Graphics;
    //fn new_with_screen_context(button: UiButtonRaw) -> Self;
}

//Debug/testing function
fn create_basic_rectangle_button() -> ButtonData {
    ButtonData {
        visible: true,
        dimensions: rectangle::rectangle_by_corners(0.0, 0.0, 2.0, 2.0),
        position_x: 10.0,
        position_y: 10.0,
        color: [1.0, 1.0, 1.0, 1.0],
        push_id: Some("basic_button".to_string()),
    }
}

//Basic scaling function These are necessary due to scala's dificulty in parsing floats appropriately
fn scale_for_f32(x: u64) -> f32 {
    (x as f32) / 100.0
}
fn scale_for_f64(x: i32) -> f64 {
    x as f64 / 100.0
}


//Functionality tests.
#[test]
fn test_basic_button_translation() {
    //Todo: Add Manual Button Creation
    let button = UiButtonRaw::read("assets/GUI/example_button.xml");
    let x = ButtonData::new(button.unwrap());
}

#[test]
fn test_cursor_triggers_inside_button() {
    let button = create_basic_rectangle_button();
    assert_eq!(button.is_inside([11.0, 11.0]), true);
}

#[test]
fn test_cursor_triggers_outside_button() {
    let button = create_basic_rectangle_button();
    assert_eq!(button.is_inside([0.0, 0.0]), false);
}

#[test]
fn test_cursor_triggers_on_position_corner() {
    let button = create_basic_rectangle_button();
    assert_eq!(button.is_inside([10.0, 10.0]), true);
    assert_eq!(button.is_inside([12.0, 12.0]), true);
    assert_eq!(button.is_inside([10.0, 12.0]), true);
    assert_eq!(button.is_inside([12.0, 10.0]), true);
}
