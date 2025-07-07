#[derive(Copy, Clone, Debug)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;
    fn next(&mut self) -> Option<<Self>::Item> {
        let curr = self.clone();

        if self.v <= 1 {
            return None;
        }
        if self.v % 2 == 0 {
            self.v = self.v / 2;
        } else {
            self.v = (self.v * 3) + 1;
        }
        Some(curr)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut i = 0;
    let mut n = Collatz::new(n);
    while let Some(_) = n.next() {
        i += 1;
    }
    i
}