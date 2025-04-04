use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }

    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
    
    pub fn intersect(self, circle: Circle) -> bool {
        self.center.distance(circle.center) <= (self.radius + circle.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, point: Point) -> f64 {
        ((self.0 - point.0).powi(2) + (self.1 - point.1).powi(2)).sqrt()
    }
}