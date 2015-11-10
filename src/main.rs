extern crate image;
extern crate rand;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
#[derive(Copy, Clone)]
struct element {
    stickiness: f64,
}
impl element {
    fn Hydrogen() -> element {
        return element {
            // name: "Hydrogen".to_string(),
            stickiness: 0.5,
        };
    }
}

#[derive(Copy, Clone)]
struct particle {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    elementType: element,
}
impl particle {
    //    fn time_step(self) {
    // self.x = self.x + self.vx;
    // self.y = self.y + self.vy;
    // self.z = self.z + self.vz;
    // }
    fn time_step_return(&self) -> particle {
        return particle {
            x: self.x + self.vx,
            y: self.y + self.vy,
            z: self.z + self.vz,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            elementType: self.elementType,
        };
    }
    fn distance(&self, other: &particle) -> f64 {
        return ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) +
                (other.z - self.z).powi(2))
                   .sqrt();
    }
    fn did_collide(&self, other: &particle) -> bool {
        return self.distance(other).abs() < (self.radius + other.radius);
    }
    fn will_stick(&self, other: &particle) -> bool {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen_range(0.0, 1.0);
        return rand > (self.elementType.stickiness + other.elementType.stickiness / 2.0);
    }
    fn sticky_collision(&self, other: &particle) -> particle {
        return particle {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx + other.vx,
            vy: self.vy + other.vy,
            vz: self.vz + other.vz,
            radius: self.radius + (other.radius / 2.0),
            elementType: element {
                stickiness: (self.elementType.stickiness + other.elementType.stickiness) / 2.0,
            },
        };
    }
    fn bounds_check(self, height: f64, width: f64, length: f64) -> particle{
        let mut return_particle:particle = self.clone();

        if self.z > height  || self.x < 0.0 {
           return_particle.z = height ;
            return_particle.vz = (-self.vz);
        }
        if self.y > length  || self.y < 0.0 {
            return_particle.y = length;
            return_particle.vy = (-self.vy);
        }
        if self.x > width  || self.x < 0.0 {
            return_particle.x = width;
            return_particle.vx = (-self.vx);
        }
        return return_particle;
    }
}

fn main() {
    let height = 2;
    let width = 100;
    let length = 200;
    let initvx = 1.0;
    let initvy = 1.0;
    let initvz = 1.0;
    let time_steps = 5;
    let distribution = 2; // units per particle
    let mut field: Vec<particle> = Vec::new();
    for z in 0..height {
        for x in 0..width {
            for y in 0..length {
                if height % distribution == 0 {
                    field.push(particle {
                        x: x as f64,
                        y: y as f64,
                        z: z as f64,
                        vx: initvx,
                        vy: initvy,
                        vz: initvz,
                        radius: 1.0,
                        elementType: element::Hydrogen(),
                    });
                }
            }
        }
    }
for frame in 0..time_steps {
    // Time Step
    for i in 0..field.len() {
        field[i] = field[i].time_step_return();
    }
    // Check for collisions
    let mut j_was_hit: bool = false;
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    loop {
        loop {
            field[i] = field[i].bounds_check(height as f64, width as f64, length as f64);
            field[j] = field[j].bounds_check(height as f64, width as f64, length as f64);
            if field[i].did_collide(&field[j]) {
                if field[i].will_stick(&field[j]) {
                    field[i] = field[i].sticky_collision(&field[j]);
                    field.swap_remove(j);
                }
                println!("particle {} was hit by particle {}", i, j);
                j_was_hit = true;
            }
            println!("particle {} was not hit by particle {}", i, j);
            j_was_hit = false;
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
            for j in 0..2 * field[part].radius as usize { /*TODO fix this nonsense*/
            let mut i = j - field[part].radius as usize;
                if !(field[part].x + i as f64 >= width as f64) && !(field[part].y as f64 >= length as f64) {
        img.get_pixel_mut((field[part].x + i as f64) as u32, (field[part].y as f64) as u32).data = [255, 255, 255];
            }
        if !(field[part].x as f64 >= width as f64) && !(field[part].y + i as f64 >= length as f64) {
        img.get_pixel_mut((field[part].x  as f64) as u32, (field[part].y + i as f64) as u32).data = [255, 255, 255];
        }
        }
    }
    img.save(format!("frame{}_output.png", frame));
    }    

}
