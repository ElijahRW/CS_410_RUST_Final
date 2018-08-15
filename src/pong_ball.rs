//Potential Trig velocity to use: http://docs.piston.rs/mush/float/trait.Trig.html
//Radians: http://docs.piston.rs/mush/float/trait.Radians.html

//extern crate float;
//extern crate find_folder;
extern crate find_folder;
extern crate piston_window;

//extern crate mush;

//use num_traits::Float;
use self::piston_window::*;

use std::f64;

//use self::piston_window::*;
//use self::mush::std;
//use self::vecmath::*;

//use piston_translator::*;

//use self::m

//#[derive(debug)]
pub struct PongBall {
    pub x: f64,
    pub y: f64,
    velocity: Velocity,
    pub dimensions: types::Rectangle,
    pub color: [f32; 4],
}

impl PongBall {
    pub fn default_new() -> Self {
        PongBall {
            x: 150.0,
            y: 150.0,
            velocity: Velocity::new(),
            dimensions: rectangle::rectangle_by_corners(0.0, 0.0, 10.0, 10.0),
            color: [1.0, 1.0, 0.0, 1.0],
        }
    }

    pub fn draw<G>(&self, transform: math::Matrix2d, g: &mut G) where G: Graphics, {
            Rectangle::new(self.color).draw(self.dimensions, &Default::default(), transform, g);
    }

    pub fn move_ball(&mut self) {
        //TODO: push down cos/sin functionality to Velocity
        self.x = self.x + self.velocity.direction_deg.sin() * self.velocity.speed; //TODO: Get sin functionality (all that's needed here)
        self.x = self.x + self.velocity.direction_deg.cos() * self.velocity.speed;
        //println!("Ball Position: x:{}, y:{}", self.x, self.y)
    }
    pub fn rotate_direction(&mut self, angle_deg: f64) {
        self.velocity.rotate_direction(angle_deg);
    }
}

struct Velocity {
    speed: f64,
    direction_deg: f64, //we will just use an angle representation for now
}

impl Velocity {
    pub fn new() -> Self {
        Velocity {
            speed: 5.0,
            direction_deg: (186.0 as f64).to_radians(), //we'll use a
        }
    }
    pub fn rotate_direction(&mut self, angle_deg: f64) {
        self.direction_deg = (self.direction_deg + angle_deg) % (360.0 as f64).to_radians();
    }
    pub fn flat_colision(&mut self) {
        self.direction_deg = 0.0;
    }
}
