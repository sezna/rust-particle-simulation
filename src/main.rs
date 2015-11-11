extern crate image;
extern crate rand;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
#[derive(Copy, Clone)]
struct Element {
    stickiness: f64,
}
impl Element {
    fn hydrogen() -> Element {
        return Element {
            // name: "hydrogen".to_string(),
            stickiness: 0.5,
        };
    }
}

#[derive(Copy, Clone)]
struct Particle {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    element_type: Element,
}
fn distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    return (((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2))).sqrt() as u32;   
}
impl Particle {
    fn low_energy_particle(x: f64, y: f64, z: f64) -> Particle {
        let mut rng = thread_rng();
        return Particle {
            x: x,
            y: y,
            z: z,
            vx: rng.gen_range(-0.5, 0.5),
            vy: rng.gen_range(-0.5, 0.5),
            vz: rng.gen_range(-0.5, 0.5),
            radius: rng.gen_range(0.3, 1.0),
            element_type: Element::hydrogen(),
        }
        
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
            element_type: self.element_type,
        };
    }
    fn distance(&self, other: &Particle) -> f64 {
        return ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) +
                (other.z - self.z).powi(2))
                   .sqrt();
    }
    fn did_collide(&self, other: &Particle) -> bool {
        return self.distance(other).abs() < (self.radius + other.radius);
    }
    fn will_stick(&self, other: &Particle) -> bool {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen_range(0.0, 1.0);
        return rand > (self.element_type.stickiness + other.element_type.stickiness / 2.0);
    }
    fn sticky_collision(&self, other: &Particle) -> Particle {
        return Particle {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx + other.vx,
            vy: self.vy + other.vy,
            vz: self.vz + other.vz,
            radius: self.radius + (other.radius / 2.0),
            element_type: Element {
                stickiness: (self.element_type.stickiness + other.element_type.stickiness) / 2.0,
            },
        };
    }
    fn bounds_check(self, height: f64, width: f64, length: f64) -> Particle{
        let mut return_particle:Particle = self.clone();

        if self.z > height  || self.x < 0.0 {
           return_particle.z = height ;
            return_particle.vz = -self.vz;
        }
        if self.y > length  || self.y < 0.0 {
            return_particle.y = length;
            return_particle.vy = -self.vy;
        }
        if self.x > width  || self.x < 0.0 {
            return_particle.x = width;
            return_particle.vx = -self.vx;
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
    let height = 2;
    let width = 50;
    let length = 600;
    let time_steps = 20;
    let distribution = 0.5; // units per Particle
    let mut field: Vec<Particle> = Vec::new();
    for z in 0..height {
        for x in 0..width {
            for y in 0..length {
                    let mut rng = thread_rng();
                    let maybe = rng.gen_range(0.0, 1.0);
                    if maybe > distribution {
                    field.push(Particle::low_energy_particle(x as f64, y as f64, z as f64));
                    }
                    
            }
        }
    }
for frame in 0..time_steps {
    println!("time step: {}", frame);
    // Time Step
    for i in 0..field.len() {
        field[i] = field[i].time_step_return();
    }
    // Check for collisions
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    loop {
        loop {
            field[i] = field[i].bounds_check(height as f64, width as f64, length as f64);
            field[j] = field[j].bounds_check(height as f64, width as f64, length as f64);
            if field[i].did_collide(&field[j]) {
                if field[i].will_stick(&field[j]) {
                    field[i] = field[i].sticky_collision(&field[j]);
                    field.remove(j);
                }
                else {
                    let (particle1, particle2) = field[i].elastic_collision(&field[j]);
                    field[i] = particle1;
                    field[j] = particle2;
//                println!("Particle {} was hit by Particle {}", i, j);
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
     
    let mut img = ImageBuffer::<Rgb<u8>>::new(width as u32, length as u32);
    for part in 0..field.len() {
        println!("part.x: {} part.y: {}", field[part].x, field[part].y);
        let to_draw_pix:Vec<(u32, u32)> = get_circle_pix((field[part].x as u32, field[part].y as u32), field[part].radius);
        for (x, y) in to_draw_pix {
            if x < width as u32 && y < length as u32 {
            img.get_pixel_mut(x, y).data = [255, 255, 255];
            }
        }    
        }
    img.save(format!("frame{}_output.png", frame)).unwrap();
    }    

}
fn get_circle_pix((x1, y1): (u32, u32), radius: f64) -> Vec<(u32, u32)>{
    let mut return_vec:Vec<(u32, u32)> = Vec::new();
    for x2 in (x1 - (radius as u32))..(x1 + (radius as u32)) {
        for y2 in (y1 - (radius as u32))..(y1 + (radius as u32)) {
            if distance((x1, y1), (x2, y2)) < radius as u32 {
                return_vec.push((x2, y2));
            }
        }
    }
    return return_vec;
}
