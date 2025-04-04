use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // Returns the Euclidean distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    // Associated function to create a new circle from coordinates and radius
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    // Returns the diameter of the circle
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    // Returns the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    // Returns true if the circles intersect (touching or overlapping)
    pub fn intersect(&self, other: &Circle) -> bool {
        let distance = self.center.distance(&other.center);
        distance <= (self.radius + other.radius)
    }
}
