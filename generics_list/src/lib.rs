#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{head: None}
    }

    pub fn push(&mut self, value: T) {
        let node = Node{value, next : self.head.take().map(|node| Box::new(node))};
        self.head = Some(node);
    }

    pub fn pop(&mut self) {
        self.head.take().map(|node| {
            self.head = node.next.map(|node| *node);
        });
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count+=1;
            current = node.next.as_ref().map(|node| node.as_ref());
        }
        count
    }
}