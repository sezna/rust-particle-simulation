use super::element::Element;
use super::point::Point;
use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq)]
pub struct Particle {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point,
    gravitational_acceleration: Point,
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

// TODO change f64 coords to u64
// use default constructors
// max speed constant
// friction constant

impl Particle {
    fn random_particle(x: f64, y: f64, z:f64) -> Particle {
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
            acceleration: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64
            },
            gravitational_acceleration: Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            },
            radius: rng.gen_range(3.3, 5.4),
            element: Element::hydrogen(),
        };
    }
    pub fn low_energy_hydrogen(x: f64, y: f64, z: f64) -> Particle {
        Particle::random_particle(x, y, z)
    }

    pub fn low_energy_oxygen(x: f64, y: f64, z: f64) -> Particle {
        let mut part = Particle::random_particle(x, y, z);
        part.element = Element::oxygen();
        return part;
    }

    pub fn low_energy_carbon(x: f64, y: f64, z: f64) -> Particle {
        let mut part = Particle::random_particle(x, y, z);
        part.element = Element::carbon();
        return part;
    }


    pub fn time_step_return(&self, height: f64, width: f64, length: f64) -> Particle {
        let mut to_return = self.clone();
        to_return.position = self.position.add(&self.velocity);
        to_return.velocity = self.velocity.add(&self.acceleration).add(&self.gravitational_acceleration);
        if to_return.velocity.x > 5f64 { to_return.velocity.x = 5f64; }
        if to_return.velocity.y > 5f64 { to_return.velocity.y = 5f64; }
        if to_return.velocity.z > 5f64 { to_return.velocity.z = 5f64; }
        if to_return.velocity.x < -5f64 { to_return.velocity.x = -5f64; }
        if to_return.velocity.y < -5f64 { to_return.velocity.y = -5f64; }
        if to_return.velocity.z < -5f64 { to_return.velocity.z = -5f64; }
        return to_return.bounds_check(height, width, length);
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
            acceleration: self.acceleration.add(&other.acceleration),
            gravitational_acceleration: self.gravitational_acceleration.add(&other.gravitational_acceleration),
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

    // Calculates gravity that other exerts on self.
    pub fn gravitate(&self, other: &Particle) -> Particle  {
        let grav_const = 1000f64;
        
        let x_dist = other.position.x - self.position.x;
        let y_dist = other.position.y - self.position.y;
        let z_dist = other.position.z - self.position.z;


        let g_x = grav_const * (other.radius) * (x_dist) / self.distance(other).powi(3);
        let g_y = grav_const * (other.radius) * (y_dist) / self.distance(other).powi(3);
        let g_z = grav_const * (other.radius) * (z_dist) / self.distance(other).powi(3);

        let accel_point = Point {
         x: g_x,
         y: g_y,
         z: g_z
        };

        return Particle {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
            gravitational_acceleration: accel_point,
            radius: self.radius.clone(),
            element: self.element.clone()
        };

    }


}
