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
//use ui_parser::*;
use ui_parser::*;

//





fn getSomeButtons(filePath: &str) -> Vec<UiButton> {

    //UiButton::new
    let mut array = Vec::new();

    array.push( UiButton::read(filePath).unwrap());
    //UiButton::read("assets/GUI/example_button.xml").unwrap()
    array
}

//EPRW: this file simply runs a hello world window using the piston engine.
fn main() {

    //TODO: TEMP BUTTON Parser

    //TODO: Implement parsed window settings
    let mut window: PistonWindow = WindowSettings::new(
        "EPRW UI Parse Test",
        [500, 500]
    )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let uiBUttons = getSomeButtons("assets/GUI/example_button.xml");

    window.set_lazy(true);
    while let Some(e) = window.next() {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.5];
        let square = rectangle::square(0.0, 0.0, 50.0);
        //attempt to predefine the square:
        let recy = Rectangle::new(RED);
        let recy2 = Rectangle::new([0.0,1.0,0.0,0.0]);

        window.draw_2d(&e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            g.clear_stencil(0);
            let draw_state = c.draw_state.blend(Blend::Alpha);

            //TODO: EXPLANATION: Piston has a very odd method for rendering objects such as rectangles:
                // the rectangle object itself defines the color and drawing methodes, while the input array defines the dimentions.
            recy
                .draw([25.0, 25.0, 100.0, 100.0], &draw_state, c.transform, g);



            Rectangle::new(BLUE)
                .draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

            /*//let transform = c.transform.trans(100.0, 100.0);
            // Compute clip rectangle from upper left corner.
            let (clip_x, clip_y, clip_w, clip_h) = (100, 100, 100, 100);
            let (clip_x, clip_y, clip_w, clip_h) =
                (clip_x, c.viewport.unwrap().draw_size[1] - clip_y - clip_h, clip_w, clip_h);
            let clipped = c.draw_state.scissor([clip_x, clip_y, clip_w, clip_h]);
            Image::new().draw(&rust_logo, &clipped, transform, g);

            let transform = c.transform.trans(200.0, 200.0);
            Ellipse::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 50.0, 50.0], &DrawState::new_clip(), transform, g);
            Image::new().draw(&rust_logo,
                              &if clip_inside { DrawState::new_inside() }
                                  else { DrawState::new_outside() },
                              transform, g);*/
        });
        //Simple rectangle

        //looping rectangles:
        //let rectangles:Vec<piston_window::Rectangle> = Vec::new();
    }
}

/* END OF COPIED SOURCES */