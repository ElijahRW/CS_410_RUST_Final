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
    pub push_id: Option<String>,
}


#[test]
fn test_basic_button_translation () {
    //Todo: add manual Button Creation
    let button = UiButton::read("assets/GUI/example_button.xml");
    let x = ButtonData::new(button.unwrap());
}

#[test]
fn test_cursor_triggers_inside_button() {
    let button = create_basic_button();
    assert_eq!(button.is_inside([11.0,11.0]), true);
}

#[test]
fn test_cursor_triggers_outside_button() {
    let button = create_basic_button();
    assert_eq!(button.is_inside([0.0,0.0]), false);
}
#[test]
fn test_cursor_triggers_on_position_corner() {
    let button = create_basic_button();
    assert_eq!(button.is_inside([10.0,10.0]), true);
    assert_eq!(button.is_inside([12.0,12.0]), true);
    assert_eq!(button.is_inside([10.0,12.0]), true);
    assert_eq!(button.is_inside([12.0,10.0]), true);
}



//TODO: it will be necessary to add fluid button transitions (allowing the translator to read in multiple types of buttons
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
            push_id: button.push_id,
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

    //TODO: This may need to be revised if we rearrange how rectangle dimensions are defined
    pub fn is_inside(&self, [x,y]: [f64; 2]) -> bool {
        //Is Above bottom, and below top
        let x_test = self.position_x + self.dimensions[2];
        let y_test = self.position_y + self.dimensions[3];
        println!("LOCATION DEBUG: \n\tX:{},{},\n\tY:{},{}\nMouse Vars: {}, {} ", self.position_x, x_test, self.position_y, y_test, x, y);

        if x >= self.position_x && (x <= (self.position_x + self.dimensions[2])) {
            if (y >= self.position_y) && (y <= self.position_y + self.dimensions[3]) {
                println!("BUTTON CLICKED!!!!!");
                return true;
            }
        }
        false
    }
}


fn create_basic_button() -> ButtonData {
    ButtonData {
        dimensions: rectangle::rectangle_by_corners(0.0,0.0,2.0,2.0),
        position_x: 10.0,
        position_y: 10.0,
        color: [1.0,1.0,1.0,1.0],
        push_id: Some("basic_button".to_string()),
    }
}

/*
    The Goal is to implement an iterator of items (Can't be exclusively of one type that the main logic loop may rapidly iterate through and add to the draw logic without significant loss of performance
*/
