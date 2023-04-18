mod err;
use err::{ ParseErr, ReadErr };
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
pub use std::error::Error;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;
        let json_value: TodoList = serde_json::from_str(&content)?;

        if json_value.tasks.is_empty(){
            return Err(Box::new(ParseErr::Empty));
        }else{
            Ok(json_value)
        }
    }
}