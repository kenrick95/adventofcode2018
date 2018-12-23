use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Point(/* y */ usize, /* x */ usize);

#[derive(Hash, Debug, Copy, Clone, PartialEq)]
enum Type {
  Rocky,
  Wet,
  Narrow,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
enum Tool {
  Torch,
  Gear,
  None,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
  cost: usize,
  position: Point,
  tool: Tool,
}

// Rust's BinaryHeap is max-heap. This is sample code from rust doc to make it min-heap
impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    other
      .cost
      .cmp(&self.cost)
      .then_with(|| self.position.0.cmp(&other.position.0))
      .then_with(|| self.position.1.cmp(&other.position.1))
  }
}
impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

const MOD: usize = 20183;

fn generate_map(start: &Point, end: &Point, target: &Point, depth: usize) -> Vec<Vec<Type>> {
  let mut map_values: Vec<Vec<usize>> = vec![];
  {
    // Init blank map
    let mut tmp: Vec<usize> = vec![];
    tmp.resize(end.1 + 1, 0);
    map_values.resize(end.0 + 1, tmp);
  }
  for i in start.0..(end.0 + 1) {
    for j in start.1..(end.1 + 1) {
      let value;
      if i == start.0 && j == start.1 {
        value = 0;
      } else if i == target.0 && j == target.1 {
        value = 0;
      } else if i == 0 {
        value = (j % MOD * 16807 % MOD) % MOD;
      } else if j == 0 {
        value = (i % MOD * 48271 % MOD) % MOD;
      } else {
        value = (map_values[i - 1][j] % MOD * map_values[i][j - 1] % MOD) % MOD;
        // if i == 1 && j == 1 {
        //   println!("{} {}", map_values[i - 1][j] , map_values[i][j - 1]  );
        //   println!("{} --> {}", value + depth, (value + depth) % MOD % 3);
        // }
      }
      map_values[i][j] = (value % MOD + depth % MOD) % MOD;
    }
  }
  // println!("map_values: {:?}", map_values);
  let mut result = vec![];
  {
    // Init blank result
    let mut tmp: Vec<Type> = vec![];
    tmp.resize(end.1 + 1, Type::Rocky);
    result.resize(end.0 + 1, tmp);
  }
  for i in start.0..(end.0 + 1) {
    for j in start.1..(end.1 + 1) {
      let mod_result = map_values[i][j] % 3;
      let coord_type = match mod_result {
        0 => Type::Rocky,
        1 => Type::Wet,
        2 => Type::Narrow,
        _ => Type::Rocky,
      };
      result[i][j] = coord_type;
    }
  }

  return result;
}

pub fn main() {
  // let depth = 9465;
  // let target = Point(704, 13);
  let depth = 510;
  let target = Point(10, 10);
  let start = Point(0, 0);

  // Note: Need to generate map larger than start..target
  let map = generate_map(&start, &Point(target.0 * 2, target.1 * 2), &target, depth);
  // println!("Map: {:?}", map);
  {
    let mut answer = 0;
    for i in start.0..(target.0 + 1) {
      for j in start.1..(target.1 + 1) {
        let val = match map[i][j] {
          Type::Rocky => 0,
          Type::Wet => 1,
          Type::Narrow => 2,
        };
        answer += val;
      }
    }

    // 9818 is too low; 9940 is correct
    println!("Part 1: {}", answer);
  }

  // Part 2 is Dijsktra's algorithm, a harder version of it. so exciting.println!
  {
    // Priority queue, cost: time, state: position + tool --> I used only "position" as the state and that causes wrong answer ._.
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State {
      position: start,
      cost: 0,
      tool: Tool::Torch,
    });
    let mut dist: HashMap<(Point, Tool), usize> = HashMap::new();
    dist.insert((start, Tool::Torch), 0);
    let deltas: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(State {
      cost,
      position,
      tool,
    }) = pq.pop()
    {
      // println!(
      //   "state {:?} {:?} {:?} {:?}",
      //   position,
      //   cost,
      //   tool,
      //   dist.get(&(position, tool))
      // );
      if position == target && tool == Tool::Torch {
        // println!("target reached, breaking");
        break;
      }
      if cost > *dist.get(&(position, tool)).unwrap_or(&core::usize::MAX) {
        // println!("cost > dist");
        continue;
      }

      // Move to other cell
      for delta in deltas.iter() {
        let new_pos = (position.0 as i32 + delta.0, position.1 as i32 + delta.1);
        if new_pos.0 < 0
          || new_pos.1 < 0
          || new_pos.0 >= map.len() as i32
          || new_pos.1 >= map[0].len() as i32
        {
          continue;
        }
        let new_point = Point(new_pos.0 as usize, new_pos.1 as usize);
        let new_point_type = map[new_point.0 as usize][new_point.1];
        let new_point_tools_allowed = match new_point_type {
          Type::Rocky => vec![Tool::Torch, Tool::Gear],
          Type::Wet => vec![Tool::None, Tool::Gear],
          Type::Narrow => vec![Tool::Torch, Tool::None],
        };

        if new_point_tools_allowed.contains(&tool) {
          let new_cost = cost + 1;
          let next_best_dist = *dist.get(&(new_point, tool)).unwrap_or(&core::usize::MAX);

          if new_cost < next_best_dist {
            pq.push(State {
              position: new_point,
              cost: new_cost,
              tool: tool,
            });
            dist.insert((new_point, tool), new_cost);
          }
        }
      }

      // Change tool
      let point_type = map[position.0][position.1];

      let tools_allowed = match point_type {
        Type::Rocky => vec![Tool::Torch, Tool::Gear],
        Type::Wet => vec![Tool::None, Tool::Gear],
        Type::Narrow => vec![Tool::Torch, Tool::None],
      };
      for new_tool in tools_allowed.iter() {
        let new_cost = cost + 7;
        let next_best_dist = *dist
          .get(&(position, *new_tool))
          .unwrap_or(&core::usize::MAX);

        if new_cost < next_best_dist {
          pq.push(State {
            position: position,
            cost: new_cost,
            tool: *new_tool,
          });
          dist.insert((position, *new_tool), new_cost);
        }
      }
    }

    // 970 is too high; 959 is too high; 957 is also too high... something is wrong, need time to debug ... 944 is correct yay!
    println!("Part 2: {:?}", dist.get(&(target, Tool::Torch)));
  }
}
