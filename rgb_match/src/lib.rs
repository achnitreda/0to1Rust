#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
         if (self.r == first && self.g == second) || (self.g == first && self.r == second) {
            (self.r, self.g) = (self.g, self.r);
        } else if (self.r == first && self.b == second) || (self.b == first && self.r == second) {
            (self.r, self.b) = (self.b, self.r);
        } else if (self.r == first && self.a == second) || (self.a == first && self.r == second) {
            (self.r, self.a) = (self.a, self.r);
        } else if (self.g == first && self.b == second) || (self.b == first && self.g == second) {
            (self.g, self.b) = (self.b, self.g);
        } else if (self.g == first && self.a == second) || (self.a == first && self.g == second) {
            (self.g, self.a) = (self.a, self.g);
        } else if (self.b == first && self.a == second) || (self.a == first && self.b == second) {
            (self.b, self.a) = (self.a, self.b);
        }
        self
    }
}
