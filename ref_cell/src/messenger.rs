use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            max,
        }
    }

    pub fn set_value<T>(&self, rc_value: &Rc<T>) {
        let current_count = Rc::strong_count(rc_value);
        let percentage = (current_count * 100) / self.max;

        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Info: you have used up over {}% of your quota! Proceeds with precaution", 
                percentage
            ));
        }
    }

    pub fn peek<T>(&self, rc_value: &Rc<T>) {
        let current_count = Rc::strong_count(rc_value);
        let percentage = (current_count * 100) / self.max;
        
        self.logger.info(&format!("Info: you are using up to {}% of your quota", percentage));
    }
}