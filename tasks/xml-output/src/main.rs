extern crate xml;

use std::collections::HashMap;
use std::str;

use xml::writer::{EmitterConfig, XmlEvent};

fn characters_to_xml(characters: HashMap<String, String>) -> String {
    let mut output: Vec<u8> = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);

    writer
        .write(XmlEvent::start_element("CharacterRemarks"))
        .unwrap();

    for (character, line) in &characters {
        let element = XmlEvent::start_element("Character").attr("name", character);
        writer.write(element).unwrap();
        writer.write(XmlEvent::characters(line)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
    }

    writer.write(XmlEvent::end_element()).unwrap();
    str::from_utf8(&output).unwrap().to_string()
}

fn main() {
    let mut input = HashMap::new();
    input.insert(
        "April".to_string(),
        "Bubbly: I'm > Tam and <= Emily".to_string(),
    );
    input.insert(
        "Tam O'Shanter".to_string(),
        "Burns: \"When chapman billies leave the street ...\"".to_string(),
    );
    input.insert("Emily".to_string(), "Short & shrift".to_string());

    let output = characters_to_xml(input);

    println!("{}", output);
}
