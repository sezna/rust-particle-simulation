extern crate image;
extern crate rand;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use std::thread;

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
            color: [255, 255, 255],
        };
    }

    fn oxygen() -> Element {
        return Element {
            name: String::from("oxygen"),
            stickiness: 0.2,
            color: [0, 255, 100],
        };
    }
}

// TODO factor out into point_3d
// make combine names function
// utils dir, color struct
#[derive(Clone)]
struct Particle {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    element: Element,
}

fn distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    return ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32;
}

fn average_color(color_one: [u8; 3], color_two: [u8; 3]) -> [u8; 3] {
    [
        color_one[0] + color_two[0] / 2,
        color_one[1] + color_two[1] / 2,
        color_one[2] + color_two[2] / 2,
    ]
}

impl Particle {
    fn low_energy_hydrogen(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            x: x,
            y: y,
            z: z,
            vx: rng.gen_range(-0.5, 0.5),
            vy: rng.gen_range(-0.5, 0.5),
            vz: rng.gen_range(-0.5, 0.5),
            radius: rng.gen_range(0.3, 1.4),
            element: Element::hydrogen(),
        };
    }

    fn low_energy_oxygen(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            x: x,
            y: y,
            z: z,
            vx: rng.gen_range(-0.5, 0.5),
            vy: rng.gen_range(-0.5, 0.5),
            vz: rng.gen_range(-0.5, 0.5),
            radius: rng.gen_range(0.3, 1.4),
            element: Element::oxygen(),
        };
    }

    fn high_energy_oxygen(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            x: x,
            y: y,
            z: z,
            vx: rng.gen_range(-10.5, 10.5),
            vy: rng.gen_range(-10.5, 10.5),
            vz: rng.gen_range(-10.5, 10.5),
            radius: rng.gen_range(0.3, 3.4),
            element: Element::oxygen(),
        };
    }

    fn time_step_return(&self) -> Particle {
        return Particle {
            x: self.x + self.vx,
            y: self.y + self.vy,
            z: self.z + self.vz,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            element: self.element.clone(),
        };
    }
    fn distance(&self, other: &Particle) -> f64 {
        return ((other.x - self.x).powi(2)
            + (other.y - self.y).powi(2)
            + (other.z - self.z).powi(2))
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
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx + other.vx,
            vy: self.vy + other.vy,
            vz: self.vz + other.vz,
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

        if self.z > height {
            return_particle.z = height;
            return_particle.vz = -self.vz;
        }
        if self.y > length {
            return_particle.y = length;
            return_particle.vy = -self.vy;
        }
        if self.x > width {
            return_particle.x = width;
            return_particle.vx = -self.vx;
        }
        if self.z < 0.0 {
            return_particle.z = 0.0;
            return_particle.vz = -self.vz;
        }
        if self.x < 0.0 {
            return_particle.x = 0.0;
            return_particle.vx = -self.vx;
        }
        if self.y < 0.0 {
            return_particle.y = 0.0;
            return_particle.vy = -self.vy;
        }
        return return_particle;
    }
    fn elastic_collision(&self, other: &Particle) -> (Particle, Particle) {
        let mut particle1 = self.clone();
        particle1.vx = other.vx;
        particle1.vy = other.vy;
        particle1.vz = other.vz;
        let mut particle2 = other.clone();
        particle2.vx = self.vx;
        particle2.vy = self.vy;
        particle2.vz = self.vz;
        return (particle1, particle2);
    }
}

fn main() {
    let height = 1;
    let width = 1000;
    let length = 1000;
    let distribution = 15.0; // units per Particle
    let mut field: Vec<Particle> = Vec::new();
    for z in 0..height {
        println!("height: {}", z);
        for x in 0..width {
            for y in 0..length {
                let mut rng = thread_rng();
                let maybe = rng.gen_range(0.0, 1.0);
                if maybe > distribution {
                    // We are adding a particle, but now we see which kind.

                    if maybe < 0.33 {
                        field.push(Particle::low_energy_hydrogen(x as f64, y as f64, z as f64));
                    } else if maybe < 0.66 {
                        field.push(Particle::low_energy_oxygen(x as f64, y as f64, z as f64));
                    } else {
                        field.push(Particle::high_energy_oxygen(x as f64, y as f64, z as f64));
                    }
                }
            }
        }
    }
    let mut frame = 0;
    loop {
        if field.len() < 5 || frame == 60 {
            break;
        }
        println!("time step: {}", frame);
        // Time Step
        for i in 0..field.len() {
            field[i] = field[i].time_step_return();
        }
        // Check for collisions
        let mut particle_coords: Vec<(i32, i32, f64)> = Vec::new();
        let mut i = 0 as usize;
        let mut j = 0 as usize;
        loop {
            particle_coords.push((field[i].x as i32, field[i].y as i32, field[i].radius));
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
                break;
            }
        }
        // Output map
        println!("drawing frame {}", frame);

        fn closurefunc(
            particles: Vec<(i32, i32, f64)>,
            frame_number: i32,
            width_local: i32,
            length_local: i32,
        ) {
            let mut img = ImageBuffer::<Rgb<u8>, _>::new(width_local as u32, length_local as u32);
            for (x, y, radius) in particles {
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
                                img.get_pixel_mut(x2, y2).data = [255, 255, 255];
                            }
                        }
                    }
                }
            }
            img.save(format!("frame{:09}_output.png", frame_number))
                .unwrap();
            return;
        };
        thread::spawn(move || closurefunc(particle_coords, frame, width, length));
        frame = frame + 1;
    }
}
