extern crate piston_window;

use piston_translator::*;
use self::piston_window::*;



pub trait PaddleControls {
    fn move_up(&mut self, velocity: f64) -> f64;
    fn move_down(&mut self, step: f64) -> f64;

    fn new_default_paddle() -> Self;
}


impl PaddleControls for ButtonData{
    fn move_up(& mut self, velocity: f64) -> f64 {
        self.position_y = self.position_y-velocity;
        self.position_y
    }

    fn move_down(&mut self, velocity: f64) -> f64 {
        self.position_y = self.position_y+velocity;
        self.position_y
    }
    fn new_default_paddle() -> Self {
        ButtonData{
            visible: true,
            dimensions: rectangle::rectangle_by_corners(
                0.0,
                0.0,
                50.0,
                200.0,
            ),
            position_x: 150.0,
            position_y: 150.0,
            color: [0.0,1.0,0.0,1.0],
            push_id: None,
        }
    }
}