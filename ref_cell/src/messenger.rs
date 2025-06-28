use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger : &'a dyn Logger,
    pub value : usize,
    pub max : usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger : &'a dyn Logger, max : usize) -> Self {
        Self{logger, value:0,max}
    }

    pub fn set_value(&self, track_value : &Rc<usize>) {
        let x = (Rc::strong_count(track_value)*100)/self.max;
        if x >= 100 {
            self.logger.error(&format!("you are over your quota!"));
        } else if x < 100 && x >= 70 {
            self.logger.warning(&format!("you have used up over {}% of your quota! Proceeds with precaution",x));
        }
    }

    pub fn peek(&self, track_value : &Rc<usize>) {
        let x = (Rc::strong_count(track_value)*100)/self.max;
        self.logger.info(&format!("you are using up to {}% of your quota",x));
    }
}