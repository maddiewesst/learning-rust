mod messenger;
use std::collections::HashMap;
pub use std::cell::RefCell;
use std::rc::Rc;
pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String,String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Worker {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::<String>::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.all_messages.borrow_mut().push(msg.to_string());
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg[9..].to_string());
    }

    fn error(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg[7..].to_string());
    }

    fn info(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg[6..].to_string());
    }


}