extern crate image;
extern crate rand;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use std::thread;


#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

struct Mass<'a> {
    particles: Vec<&'a Particle>,
    central_of_gravity: Point
}

#[derive(Clone)]
struct Element {
    stickiness: f64,
    name: String,
    color: [u8; 3],
}

impl Element {
    fn hydrogen() -> Element {
        return Element {
            name: String::from("hydrogen"),
            stickiness: 0.6,
            color: [255, 0, 0],
        };
    }

    fn oxygen() -> Element {
        return Element {
            name: String::from("oxygen"),
            stickiness: 0.2,
            color: [0, 255, 0],
        };
    }

    fn carbon() -> Element {
        return Element {
            name: String::from("carbon"),
            stickiness: 0.34,
            color: [0, 0, 255]
        }
    }
}

// TODO 
// convert f64s to u64s (?)
// make combine names function
// utils dir, color struct
// redo image making, it currently is really slow
// when a big particle eats a small one, make the position the big one 
// gravity
// elastic collision proportional to size, not just straight velocity transfer
// 
// maybe make combined particles not a circle? like stick to the outside?
#[derive(Clone)]
struct Particle {
    position: Point,
    velocity: Point,
    radius: f64,
    element: Element,
}

fn distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    return ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32;
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
    fn low_energy_hydrogen(x: f64, y: f64, z: f64) -> Particle {
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

    fn low_energy_oxygen(x: f64, y: f64, z: f64) -> Particle {
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

    fn low_energy_carbon(x: f64, y: f64, z: f64) -> Particle {
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


    fn time_step_return(&self) -> Particle {
        return Particle {
            position: self.position.add(&self.velocity),
            velocity: self.velocity.clone(),
            radius: self.radius,
            element: self.element.clone(),
        };
    }
    fn distance(&self, other: &Particle) -> f64 {
        return ((other.position.x - self.position.x).powi(2)
            + (other.position.y - self.position.y).powi(2)
            + (other.position.z - self.position.z).powi(2))
        .sqrt();
    }
    fn did_collide(&self, other: &Particle) -> bool {
        return self.distance(other).abs() < (self.radius + other.radius);
    }
    fn will_stick(&self, other: &Particle) -> bool {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen_range(0.0, 1.0);
        return rand > (self.element.stickiness + other.element.stickiness / 2.0);
    }
    fn sticky_collision(&self, other: &Particle) -> Particle {
        let new_radius = (self.radius.powi(3) + other.radius.powi(3)).cbrt();
        return Particle {
            position: self.position.clone(),
            velocity: self.velocity.add(&other.velocity),
            radius: new_radius,
            element: Element {
                name: format!("{}-{}", self.element.name, other.element.name),
                stickiness: (self.element.stickiness + other.element.stickiness) / 2.0,
                color: average_color(self.element.color, other.element.color),
            },
        };
    }

    fn bounds_check(&self, height: f64, width: f64, length: f64) -> Particle {
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
    fn elastic_collision(&self, other: &Particle) -> (Particle, Particle) {
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

fn main() {
    let height = 1;
    let width = 1000;
    let length = 1000;
    let distribution = 0.98; // units per Particle
    let mut field: Vec<Particle> = Vec::new();
    for z in 0..height {
        println!("height: {}", z);
        for x in 0..width {
            for y in 0..length {
                let mut rng = thread_rng();
                let maybe = rng.gen_range(0.0, 1.0);
                if maybe > distribution {
                    let  which_particle = rng.gen_range(0.0, 1.0);
                    // We are adding a particle, but now we see which kind.
                    if which_particle < 0.4 {
                        field.push(Particle::low_energy_hydrogen(x as f64, y as f64, z as f64));
                    } else if which_particle < 0.8 {
                        field.push(Particle::low_energy_oxygen(x as f64, y as f64, z as f64));
                    }
                    else {
                        field.push(Particle::low_energy_carbon(x as f64, y as f64, z as f64));
                    }
                }
            }
        }
    }
    let mut frame = 0;
    loop {
        if field.len() < 5 || frame == 260 {
            println!("breaking: {} {}", field.len(), frame);
            break;
        }
        println!("time step: {}", frame);
        // Time Step
        for i in 0..field.len() {
            field[i] = field[i].time_step_return();
        }
        // Check for collisions
        let mut particle_coords: Vec<(i32, i32, f64, [u8; 3])> = Vec::new();
        let mut i = 0 as usize;
        let mut j = 0 as usize;
        loop {
            particle_coords.push((field[i].position.x as i32, field[i].position.y as i32, field[i].radius, field[i].element.color));
            loop {
                field[i] = field[i].bounds_check(height as f64, width as f64, length as f64);
                field[j] = field[j].bounds_check(height as f64, width as f64, length as f64);
                if i == j {
                    break;
                }
                if field[i].did_collide(&field[j]) {
                    if field[i].will_stick(&field[j]) {
                        field[i] = field[i].sticky_collision(&field[j]);
                        field.remove(j);
                    } else {
                        let (particle1, particle2) = field[i].elastic_collision(&field[j]);
                        field[i] = particle1;
                        field[j] = particle2;
                        // println!("Particle {} was hit by Particle {}", i, j);
                    }
                }
                // println!("Particle {} was not hit by Particle {}", i, j);
                j = j + 1;
                if j >= field.len() || i >= field.len() {
                    break;
                }
            }
            j = 0 as usize;
            i = i + 1;
            if i >= field.len() || j >= field.len() {
                println!("done calculating frame");
                break;
            }
        }
        // Output map
        print!("drawing frame {}... ", frame);

        fn closurefunc(
            particles: Vec<(i32, i32, f64, [u8; 3])>,
            frame_number: i32,
            width_local: i32,
            length_local: i32,
        ) {
            let mut img = ImageBuffer::<Rgb<u8>, _>::new(width_local as u32, length_local as u32);
            for (x, y, radius, color) in particles {
                let x1 = x as u32;
                let y1 = y as u32;
                let mut lower_x = x1 as i32 - radius as i32;
                if lower_x < 0 {
                    lower_x = 0;
                }
                let mut lower_y = y1 as i32 - radius as i32;
                if lower_y < 0 {
                    lower_y = 0;
                }
                for x2 in lower_x as u32..(x1 + (radius as u32)) {
                    for y2 in lower_y as u32..(y1 + (radius as u32)) {
                        if distance((x1, y1), (x2, y2)) < radius as u32 {
                            if x2 < width_local as u32 && y2 < length_local as u32 {
                                img.get_pixel_mut(x2, y2).data = color ;
                            }
                        }
                    }
                }
            }
            println!("saving  frame...");
            img.save(format!("frame{:09}_output.png", frame_number))
                .unwrap();
            return;
        };
//        thread::spawn(move || closurefunc(particle_coords, frame, width, length));
        closurefunc(particle_coords, frame, width, length);
        frame = frame + 1;
    }
}
