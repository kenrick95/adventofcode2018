use regex::Regex;
use std::io;

#[derive(Debug)]
struct Particle {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Particle {
    fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

// Note: The solution for this day is sooo hacky that it requires running this code twice, by uncommenting the correct part of the code and tweaking the parameters. I don't like this, but the "proper" solution ("OCR") doesn't really worth implementing
// Idea of current solution:
// 1. Find the time "t" where all the particles are closest to one another. "Close" metric is defined by using Manhattan distance. The "loop" breaks at the next tick where the closeness starts to increase.
// 2. Print out the map, while panning to the correct section where all the particles appear.
// Idea of proper solution:
// 1. Find the "t" (as per above)
// 2. Find where the particles are
// 3. Do character recognition
pub fn main() {
    let re = Regex::new(
        r"position=<\s*([-0-9]+)\s*,\s*([-0-9]+)\s*> velocity=<\s*([-0-9]+)\s*,\s*([-0-9]+)\s*>",
    )
    .unwrap();

    let mut particles: Vec<Particle> = vec![];

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }

        let caps = re.captures(input.as_str()).unwrap();

        let x = String::from(caps.get(1).unwrap().as_str()).parse().unwrap();
        let y = String::from(caps.get(2).unwrap().as_str()).parse().unwrap();
        let vx = String::from(caps.get(3).unwrap().as_str()).parse().unwrap();
        let vy = String::from(caps.get(4).unwrap().as_str()).parse().unwrap();

        particles.push(Particle { x, y, vx, vy });
    }

    println!("Len: {}", particles.len());

    let mut t = 0;
    const MAP_SIZE: usize = 100;

    // Note: Find the "t" when the particles converged, i.e. Manhattan distance to one another is at minimum
    // let mut prev_total_distance: u128 = 0;
    // let mut total_distance: u128 = 0;
    // let mut is_converging = true;

    // while is_converging {
    //     // tick all particles
    //     particles.iter_mut().for_each(|p| {
    //         p.tick();
    //         // if 0 <= p.x && p.x < MAP_SIZE as i32 && 0 <= p.y && p.y < MAP_SIZE as i32 {
    //         //     map[p.y as usize][p.x as usize] = true;
    //         // }
    //     });

    //     // Determine if particles are converging into one another

    //     total_distance = 0;
    //     particles.iter().for_each(|p| {
    //         particles.iter().for_each(|q| {
    //             total_distance += (p.x - q.x).abs() as u128 + (p.y - q.y).abs() as u128;
    //         });
    //     });
    //     if prev_total_distance == 0 {
    //         prev_total_distance = total_distance;
    //     }

    //     if total_distance > prev_total_distance {
    //         is_converging = false;
    //     }

    //     prev_total_distance = total_distance;
    //     t += 1;
    //     println!("t={}, total_distance={}", t, total_distance);
    // }


    // Note: on test data, minimum total distance is achieved at t=3
    // Note: on real data, minimum total_distance=3188372 is achieved at t=10086
    while t < 10086 {
        particles.iter_mut().for_each(|p| p.tick());
        t += 1;
    }
    println!("particles 0 {:?}", particles[0]);

    // Note: Magic number derived (guessed, experimented) from where particle-0 is located
    let offset_x = -150;
    let offset_y = -150;
    let mut map: Vec<Vec<bool>> = vec![];
    {
        let mut tmp = Vec::new();
        tmp.resize(MAP_SIZE, false);
        map.resize(MAP_SIZE, tmp);
    }
    particles.iter().for_each(|p| {
        let px = p.x + offset_x;
        let py = p.y + offset_y;
        if 0 <= px && px < MAP_SIZE as i32 && 0 <= py && py < MAP_SIZE as i32 {
            map[py as usize][px as usize] = true;
        }
    });

    // Print map
    {
        let mut i = 0;
        while i < map.len() {
            let mut j = 0;
            while j < map[i].len() {
                if map[i][j] {
                    print!("#");
                } else {
                    print!(".");
                }
                j += 1;
            }
            println!();
            i += 1;
        }
    }
    // Note: on real data, Answer is GFANEHKJ; "OCR"-ed by human brain from characters printed in console
}
