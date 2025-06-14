#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius : f64
}

impl Circle {
    pub fn new(x : f64, y: f64, radius: f64) -> Self {
        Self {
            center : Point(x,y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius*2.0
    }

    pub fn area(&self) -> f64 {
        (self.radius*self.radius)*std::f64::consts::PI
    }

    pub fn intersect(&self, c: Circle) -> bool {
        let d = self.center.distance(c.center);
        let r1 = self.radius;
        let r2 = c.radius;

        d <= r1 + r2 && d >= (r1 - r2).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, p : Point) -> f64 {
        ((p.0-self.0).powf(2.0)+(p.1-self.1).powf(2.0)).sqrt()
    }
}