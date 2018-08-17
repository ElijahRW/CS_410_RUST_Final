//References: http://docs.piston.rs/piston_window/png/index.html
//http://docs.piston.rs/piston_window/image/index.html

//Translator Module, This file currently must be ENGINE specific. It will rely on the syntax defined
// in parser.rs
// This translator should hopefully be as LIGHT as possible, since our current implementation requires
//that a translator module exists for every potential engine.
extern crate find_folder;
extern crate piston_window;
//use self::math::{multiply, translate, Matrix2d, Vec2d, Scalar};
use self::piston_window::*;
use ui_parser::*;

pub struct ButtonData {
    pub visible: bool,
    pub dimensions: types::Rectangle,
    pub position_x: f64,
    pub position_y: f64,
    pub color: [f32; 4],
    pub push_id: Option<String>,
}

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

//TODO: it will be necessary to add fluid button transitions (allowing the translator to read in multiple types of buttons
impl UiObject for ButtonData {
    //TODO: This may need to be revised if we rearrange how rectangle dimensions are defined
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

    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G)
    where
        G: Graphics,
    {
        /*//Inlined from
            let trans = translate([self.position_x, self.position_y]);
            multiply(self, trans);*/
        if self.visible {
            Rectangle::new(self.color).draw(self.dimensions, &Default::default(), transform, g);
        }
    }
}

impl XmlButtonReadable for ButtonData {
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
            ), //TODO: Revise logic to accept centered Button (For now logic will be kept as is.
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
        //result.dimensions.set();
    }

    fn new_with_screen_context(button: UiButtonRaw, size: &Size) -> Self {
        //Todo: Create
        let style = button.location.style.clone();
        let mut value = Self::new(button);
        match style.as_ref() {
            "centered" => value.center_button(size),
            "right" => value.left_button(size),
            _ => (),
        };
        /*if button.location.style.eq("centered") || button.location.style.eq("center") {
            value.center_button(window);
        }
        if button*/
        //value.dim
        //create_basic_rectangle_button() //TODO: Placeholder function until Will be filled
        value
    }
    fn read_vec_from_file(file_path: &str) -> Vec<ButtonData> {
        let buttons = Buttons::read(file_path);
        let mut result = Vec::new();
        let button_vector = buttons.unwrap(); //TODO: add correct match case.

        for button in button_vector.buttons {
            result.push(Self::new(button));
        }
        result
    }
    fn read_from_file_w_context(file_path: &str, size: &Size) -> Vec<ButtonData> {
        let buttons = Buttons::read(file_path);
        let mut result = Vec::new();
        let button_vector = buttons.unwrap(); //TODO: add correct match case.

        for button in button_vector.buttons {
            result.push(Self::new_with_screen_context(button, size));
        }
        result
    }

    fn center_button(&mut self, size: &Size) {
        //let size = window.size();
        self.position_y = (size.height as f64) / 2.0;
        self.position_x = (size.width as f64) / 2.0;
    }

    fn left_button(&mut self, size: &Size) {
        println!("Making a left button!!!");
        self.position_x = (size.width as f64) - self.position_y;
    }
}

pub trait XmlButtonReadable: Sized {
    fn left_button(&mut self, size: &Size);
    fn center_button(&mut self, size: &Size);
    fn new(button: UiButtonRaw) -> Self;
    fn new_with_screen_context(button: UiButtonRaw, size: &Size) -> Self;
    fn read_vec_from_file(file_path: &str) -> Vec<Self>;
    fn read_from_file_w_context(file_path: &str, size: &Size) -> Vec<ButtonData>;
}

pub trait UiObject {
    //fn new(button: UiButtonRaw) -> Self;
    fn is_inside(&self, point: [f64; 2]) -> bool;
    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G)
    where
        G: Graphics;
    //fn new_with_screen_context(button: UiButtonRaw) -> Self;
}

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

fn scale_for_f32(x: u64) -> f32 {
    (x as f32) / 100.0
}
fn scale_for_f64(x: i32) -> f64 {
    x as f64 / 100.0
}

/*
    The Goal is to implement an iterator of items (Can't be exclusively of one type that the main logic loop may rapidly iterate through and add to the draw logic without significant loss of performance
*/
