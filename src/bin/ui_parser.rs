#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::deserialize;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//Parser Module used to grab ui variables from XML and translates them into simple structures.
/*
 * Button Struct
 *
*/

#[derive(Serialize, Deserialize, Debug)]
struct Button {
    location: Button_Location,
}

#[derive(Serialize, Deserialize, Debug)]
struct Button_Location {
    style: String,
    x: i32,
    y: i32,
}

//Value Bar Struct

//Menu Struct?


//Impropperly configured main function. this file should be in the bin folder once propperly configured

fn main()
{}