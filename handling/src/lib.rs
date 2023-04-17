use std::fs::File;
use std::io::prelude::*;


pub fn open_or_create(file: &str, content: &str) {

    let file = File::create(file);

    file.unwrap().write_all(content.as_bytes()).unwrap();
	
}
