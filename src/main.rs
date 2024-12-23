use std::{collections::HashMap, io::{self, Write}, ops::Deref};
use cat::pet_handlers::{color_text, CatInfo};
use file_actions::store_cat_information_to_file;
use input_handlers::InputBuffer;
use serde_json::{self, json};
// use std::rc::Rc;
// use std::cell::RefCell;

pub mod file_actions;
pub mod input_handlers;
pub mod cat;

fn main() {
    // use variables to get borrowed than to create in struct
    let mut text_buffer = String::new();
    let cat_json_object = json!(r#"{}"#);
    
    // be able to use json values for both our Cat and file instance
    let colors = HashMap::from([
        ("red", [252, 97, 70]),
        ("yellow", [249, 192, 67]),
        ("green", [141, 252, 178]),
        ("blue", [70, 119, 252])
    ]);
    // println!("{}", color_text(colors.get("yellow").unwrap(), &String::from("Load file or start new file :3")));
    // println!("{}\n", color_text(colors.get("yellow").unwrap(), &String::from("[load] [new]")));

    let file_json_string = file_actions::check_json_file_value(&colors);

    // buffer to hold properties constructed where buffer does`kjA` not live longer than it
    let mut input_buffer = InputBuffer::new(
        colors,
        io::stdin(), // create stdin object
        // borrow reference to variable to not get dropped here
        &mut text_buffer, 
        cat_json_object
    );


    // check for a string object and change to be an object value to be able to 
    // insert to it rather than be read as None
    if input_buffer.json_value_object.is_string() {
        // Parse the JSON string into a serde_json::Value
        if let Ok(parsed_json) = serde_json::from_str(input_buffer.json_value_object.as_str().unwrap()) {
            input_buffer.json_value_object = parsed_json; // Replace the raw string with parsed JSON
        } else {
            panic!("Failed to parse JSON string");
        }
    }
    let mut cat_instance = if file_json_string.len() != 0 {
        serde_json::from_str::<CatInfo>(&file_json_string).expect("could not construct from file string")
    } else {
        CatInfo::default()
    };
    // let mut cat_instance = cat_instance.unwrap();
    cat_instance.print_values();

    let action_handlers: HashMap<&str, Box<dyn Fn(&mut CatInfo, &mut InputBuffer)>> = HashMap::from([
        ("change_cat_color", Box::new(|cat_instance: &mut CatInfo, input_buffer: &mut InputBuffer| {
            cat_instance.change_colors(input_buffer);
        }) as _),
        ("meow", Box::new(|cat_instance: &mut CatInfo, input_buffer: &mut InputBuffer| {
            cat_instance.increase_meow(input_buffer);
        }) as _),
        ("change_name", Box::new(|cat_instance: &mut CatInfo, input_buffer: &mut InputBuffer| {
            input_buffer.text_buffer.clear();
            std::io::stdout().flush().unwrap();
            input_buffer.std_input.read_line(input_buffer.text_buffer).expect("invalid input");
            cat_instance.change_name(input_buffer);
        }) as _),
    ]);

    loop {

        let prompt = "select action :3".to_string();
        let prompt_color = [252, 135, 80] as [u8; 3];
        let action_color = [249, 97, 67] as [u8; 3];

        // cleanup any input, but not the buffer itself 
        input_buffer.text_buffer.clear();
        std::io::stdout().flush().unwrap();
        
        println!("[{}]", color_text(&prompt_color, &prompt));
        let refer_to_color = action_handlers.iter(); // reference to each element
        for (key, _) in refer_to_color {
            print!("[{}]", color_text(&action_color, &key.to_string()));
        }
        println!("[{}]", color_text(&action_color, &"exit".to_string()));
        
        input_buffer.std_input.read_line(input_buffer.text_buffer).expect("invalid input");

        // let writer = Rc::new(RefCell::new(&mut input_buffer));
        // let reader = Rc::clone(&writer);

        // better to clone as we only need to read once, but mutate on 
        // the action method on another scope for that reference
        let mut reader = &mut input_buffer;
        let reader_text_buffer = &reader.text_buffer.clone()[0..reader.text_buffer.len() - 1];

        if reader_text_buffer == "exit" {
            break;
        }
        // println!("{}", reader_text_buffer);
        match action_handlers.get(reader_text_buffer) {
            Some(action) => {

                action.deref()(&mut cat_instance, &mut reader)
            },
            None => {
                let error_prompt = "incorrect action >:c".to_string();
                let error_prompt_color = [252, 97, 70] as [u8; 3];
                println!("{}", color_text(&error_prompt_color, &error_prompt));
                continue;
            }
        }

        input_buffer.json_value_object = serde_json::to_value(&cat_instance).expect("could not serialize to json value");

        store_cat_information_to_file(&input_buffer.json_value_object.to_string());

        cat_instance.print_values();
    }
}
