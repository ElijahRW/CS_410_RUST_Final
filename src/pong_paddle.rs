/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission
--File Description: This file extends the functionality of piston_translator.rs to add game functionality to a the ButtonData structure.
        demonstration of the universal nature of the parsing system.
*/

extern crate piston_window;
use self::piston_window::*;
use piston_translator::*;

pub trait PaddleControls {
    fn move_up(&mut self, velocity: f64) -> f64;
    fn move_down(&mut self, step: f64) -> f64;

    fn new_default_paddle() -> Self;
}

//
impl PaddleControls for ButtonData {
    fn move_up(&mut self, velocity: f64) -> f64 {
        if self.position_y > 0.0 {
            self.position_y = self.position_y - velocity;
        }
        self.position_y
    }
    //Function designed to shift a button down by a velocity factor
    fn move_down(&mut self, velocity: f64) -> f64 {
        self.position_y = self.position_y + velocity;
        self.position_y
    }
    //Hardcoded 'new' function.
    fn new_default_paddle() -> Self {
        ButtonData {
            visible: true,
            dimensions: rectangle::rectangle_by_corners(0.0, 0.0, 25.0, 150.0),
            position_x: 20.0,
            position_y: 150.0,
            color: [0.0, 1.0, 0.0, 1.0],
            push_id: None,
        }
    }
}

//Simple test, but useful when working with future features.
#[test]
fn move_up_should_shift_paddle_location() {
    let mut paddle = ButtonData::new_default_paddle();
    let paddle_copy = ButtonData::new_default_paddle();
    paddle.move_up(10.0);
    assert!(paddle.position_y < paddle_copy.position_y);
    assert!(paddle.position_y > paddle_copy.position_y - 10.001);
}

//Simple test, but useful when working with future features.
#[test]
fn move_down_should_shift_paddle_location() {
    let mut paddle = ButtonData::new_default_paddle();
    let paddle_copy = ButtonData::new_default_paddle();
    paddle.move_down(10.0);
    assert!(paddle.position_y > paddle_copy.position_y);
    assert!(paddle.position_y < paddle_copy.position_y + 10.01);
}
