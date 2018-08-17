extern crate cs_410_rust_final_project;
use cs_410_rust_final_project::ui_parser::*;

//Simple Smoke tests... all functions simply shouldn't panic. (meaning it could effectively find the corresponding file and attempt a parse.
//Simple smoke test
#[test]
fn basic_button_deserialization_test() {
    let test_button = UiButtonRaw::read("tests/test_xml/example_button.xml").unwrap();
}

//Simple smoke test
#[test]
fn button_array_deserialzation_test() {
    let test_buttons = Buttons::read("tests/test_xml/example_button_array.xml").unwrap();
    println!("Simple Button Test:");
    for button in test_buttons.buttons {
        print!("{}", button);
    }
}

//Simple smoke test
#[test]
fn color_deserialization_test() {
    let test_buttons = ButtonColor::read("tests/test_xml/example_color.xml").unwrap();
}

//Simple smoke test
#[test]
fn window_deserialization_test() {
    let test_buttons = ButtonColor::read("tests/test_xml/example_color.xml").unwrap();
    println!("Simple Color Test:");
}
