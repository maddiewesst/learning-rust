#[derive(Debug)]
pub struct Circle {
	pub center:  Point,
	pub radius:  f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point{ x, y },
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) < self.radius + other.radius
    }
}


#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
   
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let y = self.y - other.y;
        let x = self.x - other.x;
        // let xy = x * x + y * y
        (x * x + y * y).sqrt()
    }
}