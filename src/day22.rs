use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Point(/* y */ usize, /* x */ usize);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Type {
  Rocky,
  Wet,
  Narrow,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
  let depth = 9465;
  let target = Point(704, 13);
  // let depth = 510;
  // let target = Point(10, 10);
  let start = Point(0, 0);
  let map = generate_map(&start, &target, &target, depth);
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
    // Note: the exploration map can go much further than the rectangle in part 1

    let map = generate_map(&start, &Point(target.0 * 2, target.1 * 2), &target, depth);
    // Priority queue, cost: time
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    pq.push(State {
      position: start,
      cost: 0,
      tool: Tool::Torch,
    });
    let mut dist: HashMap<Point, usize> = HashMap::new();
    let mut dist_tool: HashMap<Point, Vec<Tool>> = HashMap::new();
    dist.insert(start, 0);
    dist_tool.insert(start, vec![Tool::Torch]);
    let deltas: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    // pop top
    // for adj in all adjacent nodes
    //   for tool in all tools in adj
    //     push to pq: (adj, time + 1 + switch tool ? 7 : 0)
    while let Some(State {
      cost,
      position,
      tool,
    }) = pq.pop()
    {
      println!(
        "state {:?} {:?} {:?} {:?}",
        position,
        cost,
        tool,
        dist.get(&position)
      );
      // if position == target {
      //   println!("target reached, breaking");
      //   break;
      // }
      if cost > *dist.get(&position).unwrap_or(&core::usize::MAX) {
        println!("cost > dist");
        continue;
      }
      let point_type = map[position.0][position.1];

      let tools_allowed = match point_type {
        Type::Rocky => vec![Tool::Torch, Tool::Gear],
        Type::Wet => vec![Tool::None, Tool::Gear],
        Type::Narrow => vec![Tool::Torch, Tool::None],
      };
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

        // NOTE: When entering, need to use a tool in "tools_allowed", but but, it also need to be allowed at current point!
        let new_point_tools_allowed = match new_point_type {
          Type::Rocky => vec![Tool::Torch, Tool::Gear],
          Type::Wet => vec![Tool::None, Tool::Gear],
          Type::Narrow => vec![Tool::Torch, Tool::None],
        };

        // Get the intersection between the two
        let mut new_tools: Vec<Tool> = vec![];
        for new_point_tool in new_point_tools_allowed.iter() {
          if tools_allowed.contains(&*new_point_tool) {
            new_tools.push(*new_point_tool);
          }
        }

        let new_point_best_dist = *dist.get(&new_point).unwrap_or(&core::usize::MAX);
        // if new_point == Point(12, 10) {
        //   println!(">>>> here new_tools {:?} {:?} {:?}", new_tools, new_point_type, new_point_best_dist);
        // }

        for new_tool in new_tools.iter() {
          let mut  new_cost;
          let mut final_tool = *new_tool;
          // Gotcha: if new point is target, tools allowed is only torch, but allow to change at the very end
          
          if final_tool == tool {
            new_cost = cost + 1;
            if new_point == target && final_tool != Tool::Torch {
              final_tool = Tool::Torch;
              new_cost = cost + 8;
            }
          } else {
            new_cost = cost + 8;
          }
          
          if new_cost < new_point_best_dist {
            pq.push(State {
              position: new_point,
              cost: new_cost,
              tool: final_tool,
            });
            dist.insert(new_point, new_cost);
            dist_tool.insert(new_point, vec![final_tool]);
          } else if new_cost == new_point_best_dist {
            let mut new_point_best_tools = dist_tool
              .get(&new_point)
              .unwrap_or(&vec![final_tool])
              .clone();
            if (*new_point_best_tools).contains(new_tool) {
              continue;
            } else {

              pq.push(State {
                position: new_point,
                cost: new_cost,
                tool: final_tool,
              });
              new_point_best_tools.push(final_tool);
              dist_tool.insert(new_point, new_point_best_tools);
            }
          }
        }
      }
    }

    // 970 is too high; 959 is too high; 957 is also too high... something is wrong, need time to debug ...
    println!("Part 2: {:?}", dist.get(&target));
  }
}
