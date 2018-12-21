use super::particle::Particle;
use super::point::Point;

pub struct Mass<'a> {
    particles: Vec<&'a Particle>,
    central_of_gravity: Point
}
