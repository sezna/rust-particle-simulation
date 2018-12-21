use super::element::Element;
use super::point::Point;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Particle {
    pub position: Point,
    pub velocity: Point,
    pub radius: f64,
    pub element: Element,
}

fn average_color(color_one: [u8; 3], color_two: [u8; 3]) -> [u8; 3] {
    let (r1, g1, b1) = ( color_one[0] as u16, color_one[1] as u16, color_one[2] as u16 );
    let (r2, g2, b2) = ( color_two[0] as u16, color_two[1] as u16, color_two[2] as u16 );

   return [ if r1 + r2 > 512u16 { 255u8 } else { ((r1 + r2) / 2) as u8 }, 
            if g1 + g2 > 512u16 { 255u8 } else { ((g1 + g2) / 2) as u8 },
            if b1 + b2 > 512u16 { 255u8 } else { ((b1 + b2) / 2) as u8 },
   ];
}

impl Particle {
    pub fn low_energy_hydrogen(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            position: Point {
                x: x,
                y: y,
                z: z,
            },
            velocity: Point {
                x: rng.gen_range(-0.5, 0.5),
                y: rng.gen_range(-0.5, 0.5),
                z: rng.gen_range(-0.5, 0.5),
            },
            radius: rng.gen_range(3.3, 5.4),
            element: Element::hydrogen(),
        };
    }

    pub fn low_energy_oxygen(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            position: Point {
                x: x,
                y: y,
                z: z,
            },
            velocity: Point {
                x: rng.gen_range(-0.5, 0.5),
                y: rng.gen_range(-0.5, 0.5),
                z: rng.gen_range(-0.5, 0.5),
            },
            radius: rng.gen_range(3.3, 5.4),
            element: Element::oxygen(),
        };
    }

    pub fn low_energy_carbon(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            position: Point {
                x: x,
                y: y,
                z: z,
            },
            velocity: Point {
                x: rng.gen_range(-0.5, 0.5),
                y: rng.gen_range(-0.5, 0.5),
                z: rng.gen_range(-0.5, 0.5),
            },
            radius: rng.gen_range(3.3, 5.4),
            element: Element::carbon(),
        };
    }

    

    pub fn time_step_return(&self) -> Particle {
        return Particle {
            position: self.position.add(&self.velocity),
            velocity: self.velocity.clone(),
            radius: self.radius,
            element: self.element.clone(),
        };
    }
    pub fn distance(&self, other: &Particle) -> f64 {
        return ((other.position.x - self.position.x).powi(2)
            + (other.position.y - self.position.y).powi(2)
            + (other.position.z - self.position.z).powi(2))
        .sqrt();
    }
    pub fn did_collide(&self, other: &Particle) -> bool {
        return self.distance(other).abs() < (self.radius + other.radius);
    }
    pub fn will_stick(&self, other: &Particle) -> bool {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen_range(0.0, 1.0);
        return rand > (self.element.stickiness + other.element.stickiness / 2.0);
    }
    pub fn sticky_collision(&self, other: &Particle) -> Particle {
        let new_radius = (self.radius.powi(3) + other.radius.powi(3)).cbrt();
        return Particle {
            position: if self.radius > other.radius { self.position.clone() } else { other.position.clone() },
            velocity: self.velocity.add(&other.velocity),
            radius: new_radius,
            element: Element {
                name: format!("{}-{}", self.element.name, other.element.name),
                stickiness: (self.element.stickiness + other.element.stickiness) / 2.0,
                color: average_color(self.element.color, other.element.color),
            },
        };
    }

    pub fn bounds_check(&self, height: f64, width: f64, length: f64) -> Particle {
        let mut return_particle: Particle = self.clone();

        if self.position.z > height {
            return_particle.position.z = height;
            return_particle.velocity.z = -self.velocity.z;
        }
        if self.position.y > length {
            return_particle.position.y = length;
            return_particle.velocity.y = -self.velocity.y;
        }
        if self.position.x > width {
            return_particle.position.x = width;
            return_particle.velocity.x = -self.velocity.x;
        }
        if self.position.z < 0.0 {
            return_particle.position.z = 0.0;
            return_particle.velocity.z = -self.velocity.z;
        }
        if self.position.x < 0.0 {
            return_particle.position.x = 0.0;
            return_particle.velocity.x = -self.velocity.x;
        }
        if self.position.y < 0.0 {
            return_particle.position.y = 0.0;
            return_particle.velocity.y = -self.velocity.y;
        }
        return return_particle;
    }
    pub fn elastic_collision(&self, other: &Particle) -> (Particle, Particle) {
        let mut particle1 = self.clone();
        particle1.velocity.x = other.velocity.x;
        particle1.velocity.y = other.velocity.y;
        particle1.velocity.z = other.velocity.z;
        let mut particle2 = other.clone();
        particle2.velocity.x = self.velocity.x;
        particle2.velocity.y = self.velocity.y;
        particle2.velocity.z = self.velocity.z;
        return (particle1, particle2);
    }
}
