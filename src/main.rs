/*
***Copied Code from Piston examples
***Current purpose is to provide an example window to demonstrate UI load
***comment will be revised once this source file has been adequately
***changed to reflect original content
***For original source, see:
*/

extern crate find_folder;
extern crate piston_window;

use piston_window::draw_state::Blend;

use piston_window::*;

//extern crate ui_parser;
extern crate serde;
extern crate serde_xml_rs;
#[macro_use]
extern crate serde_derive;

mod piston_translator;
mod ui_parser;
use piston_translator::*;


//TODO: Add
//EPRW: this file simply runs a hello world window using the piston engine.
fn main() {

    //TODO: Implement parsed window settings
    let mut window: PistonWindow = WindowSettings::new(
        "EPRW UI Parse Test",
        [500, 500]
    )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let ui_buttons = get_ui_buttons("assets/GUI/example_button_array.xml");

    //TODO: add custom assets path parsing.
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    //---Custom UI Buttons
    let ui_buttons = get_ui_buttons("assets/GUI/example_button_array.xml");


    window.set_lazy(false);

    let mut is_pushed = false;
    //Mouse Variable
    let mut cursor = [0.0, 0.0];

    let mut result: Vec<String> = Vec::new();

    //TODO: Window args are vital for displaying buttons based on screen size.
    while let Some(e) = window.next() {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        //const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.5];
        //let square = rectangle::square(0.0, 0.0, 50.0);
        //attempt to predefine the square:
        let recy = Rectangle::new(RED);
        //let recy2 = Rectangle::new([0.0, 1.0, 0.0, 0.0]);


        //TODO: MOUSE DEBUG INFO: Borrowed From: https://github.com/PistonDevelopers/piston-examples/blob/master/user_input/src/main.rs
        if let Some(Button::Mouse(button)) = e.press_args() {
            //println!("We have pushed a the Mouse");
            is_pushed = true;
        }
        cursor = match e.mouse_cursor(|x, y| {
            //println!("Mouse moved '{} {}'", x, y);
            [x, y]
        }) {
            Some(x) => x,
            None => cursor
        };
        //println!("Mouse moved '{} {}'", cursor[0], cursor[1]);



        //The MAIN LOGIC
        window.draw_2d(&e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            g.clear_stencil(0);




            let draw_state = c.draw_state.blend(Blend::Alpha);

            //TODO: EXPLANATION: Piston has a very odd method for rendering objects such as rectangles:
            // the rectangle object itself defines the color and drawing methodes, while the input array defines the dimentions.
           //let mut current_transform;

            //Button ID Loop
            result.clear();



                for ui_button in &ui_buttons {
                    //c.trans(1000.0, 1000.0).transform;
                    //uiButton.dimensions.
                    rectangle(ui_button.color, ui_button.dimensions, c.trans(ui_button.position_x,ui_button.position_y).transform, g);

                    if is_pushed {
                        //println!("We have pushed the mouse!");
                        if ui_button.is_inside(cursor) {
                            //println!("We have pushed the button!");
                           match ui_button.push_id {
                                None => {},
                                Some(ref x) => {println!("Pushed: {}", x); result.push(x.clone());},
                           };
                        }
                    }
                }
            /*//DEBUG DISPLAY VALUES
            for x in result {
                println!("PUSHED: {}", x);
            }*/
            is_pushed = false;
        });



        //END BORROWED CODE

        //Simple rectangle

        //looping rectangles:
        //let rectangles:Vec<piston_window::Rectangle> = Vec::new();
    }
}
/* END OF COPIED SOURCES */


/*//TODO: Remove it
fn get_button_colisions(cursor_location: [f64; 2], ui_button: ButtonData) -> Option<String> {
    //There won't be an equality
    None
}*/

//

fn get_ui_buttons(file_path: &str) -> Vec<ButtonData> {
    ButtonData::read_buttons_from_file(file_path)
}




/*cursor = match e.mouse_cursor(|x, y| {
                [x, y]
            }) {
                Some(x) => x,
                None => cursor
            };*/