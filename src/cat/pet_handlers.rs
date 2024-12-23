use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::input_handlers::InputBuffer;

const COLOR_START: &str = "\x1B[38;2"; // Color Start
const COLOR_END: &str = "\x1b[0m"; // To flush out prev settings
const DEFAULT_CAT_ICON: &str = "
     ／l、             
    （ﾟ､ ｡ ７         
      l  ~ヽ       
      じしf_,)ノ
    ";


#[derive(Debug, Deserialize,Serialize)]
pub struct CatInfo {
    name: String,
    hunger: u8,
    meows: u32,
    happiness: u8,
    cat_color: [u8; 3]
}
impl Default for CatInfo {
    fn default() -> CatInfo {
        CatInfo {
            name: "".to_string(),hunger: 0, meows: 0, happiness: 0,cat_color: [131,209,196] as [u8;3]
        }
    }

}

impl CatInfo {
    pub fn new(name: String, hunger: u8, meows: u32, happiness: u8, cat_color: [u8; 3]) -> Self{
        CatInfo {
            name,hunger,meows,happiness,cat_color
        }
    }
    pub fn print_values(&self) {
        println!("{}", color_text(&self.cat_color, &String::from(DEFAULT_CAT_ICON)));
        println!("name: {}", color_text(&self.cat_color, &self.name));
        // println!("hunger: {}", color_text(&self.cat_color, &self.hunger.to_string()));
        println!("meows: {}", color_text(&self.cat_color, &self.meows.to_string()));
        // println!("happiness level: {}", color_text(&self.cat_color, &self.meows.to_string()));
    }

    pub fn change_name(&mut self, input_buffer: &mut InputBuffer) {
        let name = input_buffer.text_buffer[0..input_buffer.text_buffer.len() - 1].to_string();
        if let Some(object) = input_buffer.json_value_object.as_object_mut() {
            object.insert("name".to_string(), name.clone().into());
        }
        self.name = name.clone();
    }


    pub fn increase_meow(&mut self, input_buffer: &mut InputBuffer) {
        if let Some(object) = input_buffer.json_value_object.as_object_mut() {
            let mut new_meow_value: i64 = 0;
            match object.get("meows") {
                Some(value) => {new_meow_value = value.as_i64().unwrap()},
                None => (),

            }
            object.insert("meows".to_string(), (new_meow_value + 1).into());
        }
        self.meows += 1;
    }

    pub fn change_colors(&mut self, input_buffer: &mut InputBuffer) {
        let color_found: &[u8; 3] = get_color_from_user(input_buffer, "select cat color");

        self.cat_color = color_found.to_owned()
    }

}

// helper method to return a color text
// @param color list containing RGB 
// @param text text we want to wrap in colors
pub fn color_text(color: &[u8; 3], text: &String) -> String {
    format!("{};{};{};{}m{}{}",COLOR_START, color[0], color[1], color[2], text, COLOR_END)
}

// get the color based on the color list in the input buffer
// @param input_buffer contains information for our buffer to color with input buffer with json 
// @param show_prompt prompt the user and detect input afterwards
fn get_color_from_user<'a, 'b>(input_buffer: &'a mut InputBuffer, show_prompt: &'b str) -> &'a [u8; 3] {
    loop {
        input_buffer.text_buffer.clear();
        std::io::stdout().flush().unwrap();
        println!("{}", show_prompt);
        let refer_to_color = input_buffer.colors.iter(); // reference to each element
        for (key, _) in refer_to_color {
            print!("[{}]", color_text(input_buffer.colors.get(key).unwrap(), &String::from(*key)));
        }
        print!("\n");
        // let mut user_color_word = String::from(&input_buffer.text_buffer);
        input_buffer.text_buffer.pop();
        input_buffer.std_input.read_line(input_buffer.text_buffer).expect("invalid input");
        // println!("{}", input_buffer.text_buffer);
        // remove last new line character given by Stdin
        let color_found = match input_buffer.colors.get(&input_buffer.text_buffer[0..input_buffer.text_buffer.len() - 1]) {
            Some(color_tuple) => {
                color_tuple
            },
            None => { 
                println!("{}", color_text(input_buffer.colors.get("red").unwrap(), &String::from("invalid color")));
                continue;
            }
        };

        // store as a vector to satisfy JSON storing
        if let Some(object) = input_buffer.json_value_object.as_object_mut() {
            object.insert("cat_color".to_string(), color_found.to_vec().into());
            return color_found
        }
    }
    
}

