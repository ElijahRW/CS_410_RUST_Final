/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission
*/

//Potential Trig velocity to use: http://docs.piston.rs/mush/float/trait.Trig.html
//Radians: http://docs.piston.rs/mush/float/trait.Radians.html
extern crate piston_window;
use self::piston_window::*;

use std::f64;

/*
**Pong ball struct is a game logic data holder. It would be utilized to hold the information for a
    pong ball's location, color, size, velocity, etc.
**Future purpose is to implement a custom xml file aswell. Button implementation would be rather straight forware.
*/
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

    //Function to draw pong ball in the piston engine.
    pub fn draw<G>(&self, transform: math::Matrix2d, g: &mut G)
    where
        G: Graphics,
    {
        Rectangle::new(self.color).draw(self.dimensions, &Default::default(), transform, g);
    }

    //Function called in game loop to calculate the trajectory of the pong ball
    pub fn move_ball(&mut self) {
        //Ball moves at
        self.x = self.x + self.velocity.direction_deg.sin() * self.velocity.speed;
        self.x = self.x + self.velocity.direction_deg.cos() * self.velocity.speed;
    }
    pub fn rotate_direction(&mut self, angle_deg: f64) {
        self.velocity.rotate_direction(angle_deg);
    }
}

//TODO: Implement pong ball's game logic (Currently not utilized).
//Simple Structure used to monitor the velocity of a pong ball
struct Velocity {
    pub speed: f64,
    pub direction_deg: f64,
}

impl Velocity {
    pub fn new() -> Self {
        Velocity {
            speed: 2.0,
            direction_deg: (45.0 as f64).to_radians(),
        }
    }
    //Function to maintin the pong ball
    pub fn rotate_direction(&mut self, angle_deg: f64) {
        self.direction_deg = (self.direction_deg + angle_deg) % (360.0 as f64).to_radians();
    }

    //Todo: Stretch Goal Add rochette functionality.
    pub fn bounce(&mut self) {
        self.direction_deg = 0.0;
    }
}

#[test]
fn rotating_360_deg_resets_angle() {
    let mut velocity = Velocity {
        speed: 0.0,
        direction_deg: 1.0,
    };
    let mut copy = velocity.rotate_direction((360.0 as f64).to_radians());
    assert!(velocity.direction_deg < 1.01);
    assert!(velocity.direction_deg > 0.99);
}
