use super::particle::Particle;
use super::point::Point;

pub struct Mass {
    particles: Vec<Particle>,
    center_of_gravity: Point
}

// TODO mass collision: break up, merge, bounce
//

impl Mass {
    pub fn time_step(&self) -> Mass {
        let mut to_return = Mass {
            particles: self.particles.iter().map(|x| x.time_step()).collect(),
            center_of_gravity: Point::default()
        };
        to_return.center_of_gravity = to_return.get_center_of_gravity();
        return to_return;
    }

    fn get_center_of_gravity(&self) -> Point {
        self.particles.iter().fold(Point::default(), |acc, x| acc.add(&x.position))
    }
}
