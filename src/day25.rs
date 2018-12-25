use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

fn read_input() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  String::from(input.trim())
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point(i32, i32, i32, i32);

fn get_distance(p1: &Point, p2: &Point) -> usize {
  ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()) as usize
}

fn visit(
  i: usize,
  color: &mut usize,
  visited: &mut Vec<usize>,
  edges_index: &HashSet<(usize, usize)>,
) {
  if visited[i] > 0 {
    return;
  }
  visited[i] = *color;
  for v in 0..visited.len() {
    if edges_index.contains(&(i, v)) {
      visit(v, color, visited, edges_index);
    }
  }
}

pub fn main() {
  let mut vertices: Vec<Point> = vec![];
  loop {
    let input = read_input();
    if input == "" {
      break;
    }
    let splits = input.split(',');
    let mut res: Vec<i32> = vec![];
    for c in splits {
      res.push((*c).parse().unwrap());
    }
    vertices.push(Point(res[0], res[1], res[2], res[3]));
  }

  // Adjacency "matrix"
  let mut edges: HashSet<(Point, Point)> = HashSet::new();
  let mut edges_index: HashSet<(usize, usize)> = HashSet::new();
  {
    for (i, v1) in vertices.iter().enumerate() {
      for (j, v2) in vertices.iter().enumerate() {
        if get_distance(v1, v2) <= 3 {
          edges.insert(((*v1).clone(), (*v2).clone()));
          edges_index.insert((i, j));
        }
      }
    }
  }

  // Tree traversal in a forest, assign a "color"
  {
    let mut color = 0;
    // color == 0 means unvisited
    let mut visited = vec![0; vertices.len()];

    for i in 0..vertices.len() {
      if visited[i] > 0 {
        continue;
      }
      color += 1;
      visit(i, &mut color, &mut visited, &edges_index);
    }

    // println!("visited: {:?}", visited);
    println!("Part 1: {}", color);
  }
}
