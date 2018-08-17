/*
--Written by Elijah Rich-Wimmer
--Written 8/16/18
--Cs Assignment: Introduction to Rust: CS 410 Final Project Submission
*/

pub use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use serde_xml_rs::deserialize;

pub use std::ops::{AddAssign, MulAssign, Neg};
pub use std::error::Error;
pub use std::fmt;
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::path::Path;

//Parser Module used to grab ui variables from XML and translates them into simple structures.

#[derive(Serialize, Deserialize, Debug)]
pub struct UiButtonRaw {
    pub visible: Option<bool>,
    pub name: String,
    pub location: ButtonLocation,
    pub dimensions: ButtonDimensions,
    pub texture: ButtonTexture,
    pub color: ButtonColor,
    pub when_pushed: Option<ButtonTexture>,
    pub push_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonDimensions {
    //#[serde(deserialize_with = "coercible")]
    pub height: u64,
    pub width: u64,
}

//ToDo: Consider removing redundant structure: Use option syntax.
#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonLocation {
    pub style: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonColor {
    //#[serde(deserialize_with = "coercible")]//Todo: Find correct syntax for Float values
    pub r: u64,
    pub g: u64,
    pub b: u64,
    pub a: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonTexture {
    file: String,
}

//TODO: insert referenced resource

#[derive(Serialize, Deserialize, Debug)]
pub struct Buttons {
    pub buttons: Vec<UiButtonRaw>,
}

impl<'b, T: Deserialize<'b>> Readable for T {
    fn read(path_str: &str) -> Option<Self> {
        let path = Path::new(path_str);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(_why) => return None,
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
//Implementing the trait
pub trait Readable: Sized {
    fn read(path_str: &str) -> Option<Self>;
}

//Display implementations for parsed data. Used for debuging purposes.
impl fmt::Display for UiButtonRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Button Name:{}\n\t, Location: {}, \n\tTexture: {}",
            self.name, self.location, self.texture
        )
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetPath {
    asset: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    id: String,
    path: String,
}

impl AssetPath {
    pub fn get_path_by_id(&self, input_id: &str) -> Option<&str> {
        for a in &self.asset {
            //println!("")
            if a.id.eq(input_id) {
                return Some(&(a.path));
            }
        }
        None
    }
}


//Window Config: Using Piston configuration as basis: http://docs.piston.rs/piston/window/index.html
#[derive(Serialize, Deserialize, Debug)]
pub struct WindowData {
    pub style: String,
    pub dimensions: ButtonDimensions,
}

#[test]
fn create_default_buton_creation() {
    create_test_uibutton();
}

fn create_test_uibutton() -> UiButtonRaw {
    UiButtonRaw {
        visible: Some(true),
        name: "testName".to_string(),
        location: create_test_button_location(),
        dimensions: create_test_button_dimensions(),
        texture: ButtonTexture {
            file: "testTexture.tiff".to_string(),
        },
        color: create_button_color(),
        when_pushed: None,
        push_id: Some("test_id".to_string()),
    }
}

//Testing Function
fn create_button_color() -> ButtonColor {
    ButtonColor {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }
}

//Testing Function
fn create_test_button_location() -> ButtonLocation {
    ButtonLocation {
        x: 00,
        y: 0,
        style: "DefautlStyle".to_string(),
    }
}

//Testing Function
fn create_test_button_dimensions() -> ButtonDimensions {
    ButtonDimensions {
        height: 10,
        width: 10,
    }
}