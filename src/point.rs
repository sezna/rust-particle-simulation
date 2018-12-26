#[derive(Clone, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
    pub fn subtract(&self, other: &Point) -> Point 
    {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
    pub fn mult(&self, coefficient: &f64) -> Point {
        Point {
            x: self.x * coefficient,
            y: self.y * coefficient,
            z: self.z * coefficient

        }
    }
    pub fn inverse(&self) -> Point {
        Point {
           x: -self.x,
           y: -self.y,
           z: -self.z
        }
    
    }
}
