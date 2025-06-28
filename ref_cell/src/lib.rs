use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;

pub mod messenger;

pub use messenger::{Logger, Tracker};

pub struct Worker {
    pub track_value: Rc<i32>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: i32) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let full_msg = format!("Warning: {}", msg);
        
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(full_msg);
    }

    fn info(&self, msg: &str) {
        let full_msg = format!("{}", msg);
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(full_msg);
    }

    fn error(&self, msg: &str) {
        let full_msg = format!("Error: {}", msg);
        
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(full_msg);
    }
}
