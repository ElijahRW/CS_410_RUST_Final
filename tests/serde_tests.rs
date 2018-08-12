/*
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

//TODO: Copy test borrowed locations

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Item {
    pub name: String,
    pub source: String,
}


#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}

#[test]
fn serde_derefferencing_correctly_test() {
    // Create a path to the desired file
    let path = Path::new("assets/GUI/deserialization_test.xml");
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

    let project: Project = deserialize(s.as_bytes()).unwrap();

    println!(
        "TEST:\n{},\n {}___{}",
        project.name, project.items[0].name, project.items[0].source
    );
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

*/
