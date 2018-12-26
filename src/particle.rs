use super::element::Element;
use super::point::Point;
use rand::{thread_rng, Rng};

static PI: f64 = 3.14159;
static C_CONST: f64 = 30000.0; // "speed of light" constant; max speed possible

#[derive(Clone, PartialEq)]
pub struct Particle {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point,
    pub radius: f64,
    pub density: f64,
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
// refactor so collisions are funcs that take two particles
// add particle field
// wrap everything in a mass struct perhaps

impl Particle {
    fn random_particle(x: f64, y: f64, z:f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            position: Point {
                x: x,
                y: y,
                z: z,
            },
            velocity: Point::default(),
            acceleration: Point::default(),
            density: 1f64,
            radius: rng.gen_range(3.3, 7.4),
            element: Element::hydrogen(),
        };
    }

    // based on sphere particles
    pub fn mass(&self) -> f64 {
        self.volume() * self.density
    }

    pub fn volume(&self) -> f64 {
         (4.0 / 3.0) * PI * self.radius.powi(3)
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
        to_return.velocity = self.velocity.add(&self.acceleration).add(&self.acceleration);
        if to_return.velocity.x > C_CONST { to_return.velocity.x = C_CONST; }
        if to_return.velocity.y > C_CONST { to_return.velocity.y = C_CONST; }
        if to_return.velocity.z > C_CONST { to_return.velocity.z = C_CONST; }
        if to_return.velocity.x < -C_CONST { to_return.velocity.x = -C_CONST; }
        if to_return.velocity.y < -C_CONST { to_return.velocity.y = -C_CONST; }
        if to_return.velocity.z < -C_CONST { to_return.velocity.z = -C_CONST; }
        return to_return.bounds_check(height, width, length);
    }

    pub fn time_step(&self) -> Particle {
        let mut to_return = self.clone();
        to_return.position = self.position.add(&self.velocity);
        to_return.velocity = self.velocity.add(&self.acceleration).add(&self.acceleration);
        if to_return.velocity.x > C_CONST { to_return.velocity.x = C_CONST; }
        if to_return.velocity.y > C_CONST { to_return.velocity.y = C_CONST; }
        if to_return.velocity.z > C_CONST { to_return.velocity.z = C_CONST; }
        if to_return.velocity.x < -C_CONST { to_return.velocity.x = -C_CONST; }
        if to_return.velocity.y < -C_CONST { to_return.velocity.y = -C_CONST; }
        if to_return.velocity.z < -C_CONST { to_return.velocity.z = -C_CONST; }
        return to_return;
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
        let mut to_return = Particle {
            position: if self.radius > other.radius { self.position.clone() } else { other.position.clone() },
            velocity: self.velocity.add(&other.velocity),
            radius: new_radius,
            acceleration: self.acceleration.add(&other.acceleration),
            density: 0f64, 
            element: Element {
                name: format!("{}-{}", self.element.name, other.element.name),
                stickiness: (self.element.stickiness + other.element.stickiness) / 2.0,
                color: average_color(self.element.color, other.element.color),
            },
        };
        to_return.density = (self.mass() + other.mass()) / to_return.volume(); 
        return to_return;
    }

    pub fn bounds_check(&self, height: f64, width: f64, length: f64) -> Particle {
        let mut return_particle: Particle = self.clone();

        if self.position.z > height {
            return_particle.position.z = height;
            return_particle.velocity.z = -self.velocity.z;
    //        return_particle.acceleration.z = -self.acceleration.z;
        }
        if self.position.y > length {
            return_particle.position.y = length;
            return_particle.velocity.y = -self.velocity.y;
     //       return_particle.acceleration.y = -self.acceleration.y;
        }
        if self.position.x > width {
            return_particle.position.x = width;
            return_particle.velocity.x = -self.velocity.x;
     //       return_particle.acceleration.x = -self.acceleration.x;
        }
        if self.position.z < 0.0 {
            return_particle.position.z = 0.0;
            return_particle.velocity.z = -self.velocity.z;
     //       return_particle.acceleration.z = -self.acceleration.z;
        }
        if self.position.x < 0.0 {
            return_particle.position.x = 0.0;
            return_particle.velocity.x = -self.velocity.x;
      //      return_particle.acceleration.x = -self.acceleration.x;
        }
        if self.position.y < 0.0 {
            return_particle.position.y = 0.0;
            return_particle.velocity.y = -self.velocity.y;
     //       return_particle.acceleration.y = -self.acceleration.y;
        }
        return return_particle;
    }

    // transfer momentum in a collision
    //
    // impulse = mass * velocity
    // change in velocity = impulse / mass
    pub fn elastic_collision(&self, other: &Particle) -> (Particle, Particle) {
        let mut particle1 = self.clone();
        let p1_momentum = self.velocity.mult(&self.mass());
        let p2_momentum = other.velocity.mult(&other.mass()); 
        let p1_impulse = p1_momentum.subtract(&p2_momentum);
        let p2_impulse = p1_impulse.inverse();
        particle1.velocity.x += p1_impulse.x / self.mass();
        particle1.velocity.y += p1_impulse.y / self.mass();
        particle1.velocity.z += p1_impulse.z / self.mass();
        let mut particle2 = other.clone();
        particle2.velocity.x = p2_impulse.x / other.mass();
        particle2.velocity.y = p2_impulse.y / other.mass();
        particle2.velocity.z = p2_impulse.z / other.mass(); 
        return (particle1, particle2);
    }

    // Calculates gravity that other exerts on self.
    pub fn gravitate(&self, other: &Particle) -> Particle  {
        let grav_const = 10f64;
        
        let x_dist = other.position.x - self.position.x;
        let y_dist = other.position.y - self.position.y;
        let z_dist = other.position.z - self.position.z;


        //TODO switch to mass
        let g_x = grav_const * other.mass() * x_dist / (self.distance(other).powi(3) * 1.5);
        let g_y = grav_const * other.mass() * y_dist / (self.distance(other).powi(3) * 1.5);
        let g_z = grav_const * other.mass() * z_dist / (self.distance(other).powi(3) * 1.5);

        let accel_point = Point {
         x: g_x,
         y: g_y,
         z: g_z
        };

        return Particle {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.add(&accel_point), 
            density: self.density.clone(),
            radius: self.radius.clone(),
            element: self.element.clone()
        };

    }


}
