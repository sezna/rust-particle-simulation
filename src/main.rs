extern crate image;
extern crate rand;
mod point;
mod mass;
mod element;
mod particle;
mod particle_field;

use point::Point;
use particle::Particle;
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng};
use std::thread;



fn distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u32 {
    return ((x1 as f64 - x2 as f64).powi(2) + (y1 as f64 - y2 as f64).powi(2)).sqrt() as u32;
}

// TODO 
// fractured collisions, make things break apar
// acceleration can degrade unless there's jerk, which should degrade
// convert f64s to u64s (?)
// make combine names function
// utils dir, color struct
// redo image making, it currently is really slow
// when a big particle eats a small one, make the position the big one 
// gravity
// elastic collision proportional to size, not just straight velocity transfer
// extra colors
// 
// maybe make combined particles not a circle? like stick to the outside?

fn main() {
    let height = 1;
    let width = 900;
    let length = 900;
    let distribution = 0.9999; // units per Particle
    let mut field: Vec<Particle> = Vec::new();
    let mut particle_one = Particle::low_energy_oxygen(450f64, 450f64, 0f64);

    particle_one.radius = 80f64;
    particle_one.velocity = Point { x: 0.0, y: 0.0, z: 0.0 };
    particle_one.density = 10000f64;
    field.push(particle_one);

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
    println!("Size of particle field: {}", field.len());
    let mut frame = 0;
    while (field.len() > 1 && frame < 500) {
        println!("time step: {}", frame);
        // Time Step
        field = field.iter().map(|x| x.time_step_return(height as f64, width as f64, length as f64)).collect();


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
                // gravity
                field[i].acceleration = Point {x: 0f64, y: 0f64, z: 0f64};
                field[i] = field[i].gravitate(&field[j]);


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
