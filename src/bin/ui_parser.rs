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
/*
 * Button Struct
 *
*/
//#[derive(Debug)]
#[test]
fn basic_button_deserialzation_test() {
    let test_button = Button::read("assets/GUI/example_button.xml");
    //println!("\n\n\n\n\n\n\n\n\n\n");
    println!("{}", test_button);
}


#[derive(Serialize, Deserialize, Debug)]
struct Button {
    name: String,
    //#[serde(rename = "location", default)]
    location: ButtonLocation,
    texture: ButtonTexture,
    when_pushed: Option<ButtonTexture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ButtonLocation {
    style: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ButtonTexture {
    file: String,
//    when_pushed: Option<ButtonTexture>,
}

//TODO: insert resource location
impl Button {
    fn read(path_str: &str) -> Self {
        let path = Path::new(path_str);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        deserialize(s.as_bytes()).unwrap()
        // `file` goes out of scope, and the "hello.txt" file gets closed
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Button Name:{}\n\t, Location: {}, \n\tTexture: {}", self.name, self.location, self.texture)    }
}
impl fmt::Display for ButtonLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Style {}, x: {}   y: {})", self.style, self.x, self.y)

    }
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
    position: WindowPosition,
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



//Value Bar Struct

//Menu Struct?


//Improperly configured main function. this file should be in the bin folder once properly located
fn main()
{}