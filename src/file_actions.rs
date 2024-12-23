use std::{collections::HashMap, fs::File, io::{Read, Write}};

use crate::cat::pet_handlers::color_text;

pub fn store_cat_information_to_file(json_content: &String) {
    let file_to_parse_to = File::create("cat.json");
    match file_to_parse_to.unwrap().write_all(json_content.as_bytes())
     {
        Ok(_) => {},
        Err(err) => {
                panic!("Could not deserialize the file, error code: {}", err)
        }
    };
}

pub fn check_json_file_value<'a>(colors: &HashMap<&str, [u8; 3]>) ->  String {
    // format to string to be read
    let mut get_string_from_file_value = String::new();

    // open file, else make the json file
    let open_file = File::open("cat.json");

    if let Ok(file) = open_file {
        let mut file_mutable = file;
        file_mutable.read_to_string(&mut get_string_from_file_value).expect("could not read content to file");
    } else {
        match File::create("cat.json") {
            Ok(_) => (),
            Err(_) => panic!("{}", color_text(colors.get("red").unwrap(), &String::from("something went wrong creating file!")))
        };
    };

    get_string_from_file_value
}
