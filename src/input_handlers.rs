use std::{collections::HashMap, io::Stdin};

use serde_json::Value;

use crate::cat::pet_handlers::CatInfo;


pub struct InputBuffer<'a> {
    pub colors: HashMap<&'a str, [u8; 3]>,
    pub std_input: Stdin,
    pub text_buffer: &'a mut String,
    pub json_value_object: Value
}

pub enum BaseBuffer<'a> {
    BufferWithUserData(InputBuffer<'a>),
    BufferWithCatData(CatInfo)
}


impl <'a> InputBuffer<'a>  {
    pub fn new(colors: HashMap<&'a str, [u8; 3]>, std_input: Stdin, text_buffer: &'a mut String, json_value_object: Value) -> Self {
        InputBuffer { colors,std_input, text_buffer, json_value_object}
    }
}
