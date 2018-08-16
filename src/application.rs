
extern crate find_folder;
extern crate piston_window;

use piston_window::draw_state::Blend;

use self::piston_window::*;

//extern crate ui_parser;
extern crate serde;
extern crate serde_xml_rs;


//use pong_ball::*;
use piston_translator::*;
use ui_parser::*;
use pong_ball::*;
//use piston_trans



pub struct Application {
    ui_buttons: Vec<ButtonData>,
    custom_paths: AssetPath,
    the_ball: PongBall,
}


impl Application {
    pub fn new() -> Self {
        Application::new_app_default_path("assets/GUI/pong_assets.xml")
    }

    pub fn new_app_default_path( path: &str) -> Self {
        let mut initialized = Application {
            ui_buttons: Vec::new(),
            custom_paths: AssetPath::new(),
            the_ball: PongBall::default_new(),
        };
        initialized.refresh_assets();
        initialized
    }

    pub fn run(& mut self) {
        self.custom_paths = AssetPath::read("assets/GUI/pong_assets.xml").unwrap();


        //TODO: Implement parsed window settings
        let mut window: PistonWindow = WindowSettings::new(
            "EPRW UI Button Test",
            [500, 500]
        )
            .exit_on_esc(true)
            //.opengl(OpenGL::V2_1) // Set a different OpenGl version
            .build()
            .unwrap();


        //TODO: add custom assets path parsing.
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        println!("{:?}", assets);
        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        //---Custom UI Buttons
        self.refresh_assets();


        window.set_lazy(false);

        let mut is_pushed = false;
        //Mouse Variable
        let mut cursor = [0.0, 0.0];
        let mut buttons_clicked: Vec<String> = Vec::new();

        //TODO: Window args are vital for displaying buttons based on screen size.
        while let Some(e) = window.next() {

            if let Some(Button::Mouse(button)) = e.press_args() {
                //println!("We have pushed a the Mouse");
                is_pushed = true;
                println!("PUSHED!!!!!!");
            }
            cursor = match e.mouse_cursor(|x, y| [x, y]) {
                Some(x) => x,
                None => cursor,
            };
            //println!("Cursor: {:?}", cursor);

            //Keyboard Logic
            match e.press_args() {
                Some(Button::Keyboard(Key::Up)) => println!("Keyboard Up!!!"),
                Some(Button::Keyboard(Key::Down)) => println!("Keyboard Down!!! Move paddle!"),
                _ => (),
            };
            if let Some(Button::Keyboard(Key::R)) = e.release_args() {
                println!("Refresh!!");
                self.refresh_assets();
            }


            //The MAIN LOGIC
            window.draw_2d(&e, |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                g.clear_stencil(0);

                //let draw_state = c.draw_state.blend(Blend::Alpha);//Blending Demonstration.

                //TODO: EXPLANATION: Piston has a very odd method for rendering objects such as rectangles:
                // the rectangle object itself defines the color and drawing methods, while the input array defines the dimentions.
                //let mut current_transform;

                //Button ID Loop
                self.the_ball.draw(c.trans(self.the_ball.x, self.the_ball.y).transform, g);
                self.the_ball.move_ball();

                //println!("Button Count: {}", self.ui_buttons.len());

                for ui_button in &(self.ui_buttons) {
                    ui_button.draw(
                        c.trans(ui_button.position_x, ui_button.position_y)
                            .transform,
                        g,
                    );
                    //println!("After Drawing!! {}", is_pushed);
                    if is_pushed {
                        //println!("We have pushed the mouse!");
                        if ui_button.is_inside(cursor) {
                            //println!("We have pushed the button!");
                            match ui_button.push_id {
                                None => {}
                                Some(ref x) => {
                                    //println!("Pushed: {}", x);
                                    buttons_clicked.push(x.clone());
                                }
                            };
                        }
                    }
                }
                if is_pushed {
                    is_pushed = false;
                }
            });
            //println!("Testing for pushed buttons");
            //Compiling buttons that have been clicked
            for x in &buttons_clicked {
                if x.eq("refresh") {
                    self.refresh_assets();
                }
                //Debug Code.
                println!("PUSHED: {}", x);
            }
            buttons_clicked.clear();

        }
    }

    fn refresh_assets(& mut self) {
        self.custom_paths = AssetPath::read("assets/GUI/pong_assets.xml").unwrap();
        self.ui_buttons = Application::get_ui_buttons(self.custom_paths.get_path_by_id("buttons").unwrap());
    }
    fn get_ui_buttons(file_path: &str) -> Vec<ButtonData> {
        ButtonData::read_from_file(file_path)
    }

}

