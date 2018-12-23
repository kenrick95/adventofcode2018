use regex::Regex;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Particle {
  x: i32,
  y: i32,
  z: i32,
  r: usize,
}

fn read_inputs() -> Vec<String> {
  let all_inputs = String::from(
    String::from(fs::read_to_string("./src/day23.in").expect("Unable to read file")).trim(),
  );
  let inputs_str: Vec<&str> = all_inputs.split('\n').collect();
  let inputs = inputs_str
    .clone()
    .iter()
    .map(|x| String::from(x.clone()))
    .collect();
  return inputs;
}

fn is_in_range(main: &Particle, test: &Particle) -> bool {
  let distance = (main.x - test.x).abs() + (main.y - test.y).abs() + (main.z - test.z).abs();
  return distance as usize <= main.r;
}

pub fn main() {
  let mut particles: Vec<Particle> = Vec::new();
  {
    let re = Regex::new(r"pos=<([-0-9]+),([-0-9]+),([-0-9]+)>, r=([-0-9]+)").unwrap();
    let inputs = read_inputs();
    for input in inputs {
      let caps = re.captures(input.as_str()).unwrap();
      let x: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
      let y: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
      let z: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
      let r: usize = caps.get(4).unwrap().as_str().parse().unwrap();
      particles.push(Particle { x, y, z, r });
    }
  }
  particles.sort_unstable_by_key(|p| p.r);
  let strongest_particle = particles.last().unwrap();
  {
    let mut answer = 0;
    for particle in particles.iter() {
      if is_in_range(strongest_particle, particle) {
        answer += 1;
      }
    }
    println!("Part 1: {:?}", answer);
  }

  // Hmm stuck at Part 2, need to find coords that are shared among most number of particles; brute force won't work cause coord range & radius is huge (>1m)
  // At Reddit, most comment hints at "Z3" solver
  // One mentioned "Bron–Kerbosch algorithm"
  // One has interesting solution, divide the numbers into smaller number, see which region wins, then decrease the division --> maybe I'll try this one
}
