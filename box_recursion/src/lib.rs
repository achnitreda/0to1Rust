#[derive(Debug,Clone)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug,Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self{grade: None}
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let node = Worker{role, name, next: self.grade.take()};
        self.grade = Some(Box::new(node));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            self.grade = node.next;
            node.name
       })
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.clone().map(|node| {
            (node.name,node.role)
       })
    }
}