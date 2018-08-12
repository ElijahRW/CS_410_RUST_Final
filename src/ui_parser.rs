#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;


//Parser Module used to grab ui variables from XML and translates them into simple structures.



/***Button Struct***/
#[test]
fn basic_button_deserialzation_test() {
    let test_button = UiButton::read("assets/GUI/example_button.xml").unwrap();
    println!("Simple Button Test:");
    print!("{}", test_button);
}


#[derive(Serialize, Deserialize, Debug)]
struct UiButton {
    name: String,
    location: ButtonLocation,
    texture: ButtonTexture,
    when_pushed: Option<ButtonTexture>,
}

//ToDo: Consider removing redundant structure: Use option syntax.
#[derive(Serialize, Deserialize, Debug)]
struct ButtonLocation {
    style: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ButtonTexture {
    file: String,
}

//TODO: insert referenced resource URL

#[derive(Serialize, Deserialize, Debug)]
struct Buttons {
    buttons: Vec<UiButton>,
}

//TODO: Implement Result Data Type instead of Option.
impl<'b, T: serde::Deserialize<'b>> Readable for T {
    fn read(path_str: &str) -> Option<Self> {

        let path = Path::new(path_str);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => return None,
            Ok(file) => file,
        };
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        if file.read_to_string(&mut s).is_err() {
            return None;
        }
        Some(deserialize(s.as_bytes()).unwrap())
    }
}



trait Readable : Sized {
     fn read(path_str: &str) -> Option<Self>;
}

impl fmt::Display for UiButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Button Name:{}\n\t, Location: {}, \n\tTexture: {}", self.name, self.location, self.texture)}
}
impl fmt::Display for ButtonLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Style {}, x: {}   y: {})", self.style, self.x, self.y)}
}
impl fmt::Display for ButtonTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(File: {})", self.file)

    }
}





//TODO: Implement functionality for optional button texture in display
/*impl fmt::Display for Option<ButtonTexture> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self
            {
                None => write!(f, ""),
                Some(&x) => write!(f, "{}", x)
            }
    }
}*/


//Window Config: Using Piston configuration as basis: http://docs.piston.rs/piston/window/index.html
#[derive(Serialize, Deserialize, Debug)]
struct Window {
    position: Position,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: i32,
    y: i32,
}

//TODO: Consider implementing universal import for all point systems. (This might be done using a monomial syntax.)
#[derive(Serialize, Deserialize, Debug)]
struct Size {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct WindowColor {
    r: u32,
    g: u32,
    b: u32,
}



//Value Bar Struct

//Menu Struct?


//Improperly configured main function. this file should be in the bin folder once properly located
fn main()
{}