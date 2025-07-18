use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers{drops: Cell::new(0), states: RefCell::new(vec![])}
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.track_worker();
        self.states.borrow_mut().push(false);
        (id, Thread::new_thread(id,c, self))
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        let states = self.states.borrow();
        if states[id] {
            return states[id]
        }else{
            false
        }
    }
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped",id);
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get()+1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid : usize,
    pub cmd : String,
    pub parent : &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(pid: usize, cmd: String, parent: &'a Workers) -> Thread {
        Thread{pid,cmd, parent}
    }
    pub fn skill(self) {}
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}