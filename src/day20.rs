use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(/* y */ i32, /* x */ i32);

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn read_input() -> String {
    let input = String::from(fs::read_to_string("./src/day20.in").expect("Unable to read file"));
    return String::from(input.trim());
}

fn get_next_vertices(
    point: Point,
    edges: &HashSet<(Point, Point)>,
    vertices: &HashSet<Point>,
) -> Vec<Point> {
    let mut result = vec![];

    let deltas = vec![Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];

    for delta in deltas {
        let next_point = point + delta;

        if vertices.contains(&next_point) && edges.contains(&(point, next_point)) {
            result.push(next_point);
        }
    }

    return result;
}

pub fn main() {
    // 2d grid, unknown |V|, unkown |E|
    // input is constructing the Vs and Es
    let mut current_point = Point(0, 0);
    let mut vertices: HashSet<Point> = HashSet::new();
    let mut edges: HashSet<(Point, Point)> = HashSet::new();
    vertices.insert(current_point);

    // Whenever '(' is encountered, push into stack;
    // whenever '|' is encoutnered, peek top;
    // whenever ')' is encountered, pop
    let mut position_stack: Vec<Point> = vec![];

    let input = read_input();
    for c in input.chars() {
        let mut new_point = current_point;
        if c == '^' || c == '$' {
            // no-op
            continue;
        }
        if c == '(' {
            position_stack.push(current_point);
            continue;
        } else if c == ')' {
            position_stack.pop();
            continue;
        } else if c == '|' {
            current_point = *position_stack.last().unwrap();
            continue;
        }

        if c == 'E' {
            new_point.1 += 1;
        } else if c == 'W' {
            new_point.1 -= 1;
        } else if c == 'S' {
            new_point.0 += 1;
        } else if c == 'N' {
            new_point.0 -= 1;
        }

        vertices.insert(new_point);
        edges.insert((new_point, current_point));
        edges.insert((current_point, new_point));

        current_point = new_point;
    }

    // println!("Vertices: {}, {:?}", vertices.len(), vertices);
    // println!("Edges: {}, {:?}", edges.len(), edges);

    // BFS to find furtest room
    let mut distances: HashMap<Point, usize> = HashMap::new();
    for v in vertices.iter() {
        distances.insert(*v, core::usize::MAX);
    }
    distances.insert(Point(0, 0), 0);

    let mut point_queue: VecDeque<Point> = VecDeque::new();
    point_queue.push_back(Point(0, 0));

    while point_queue.len() > 0 {
        let point = point_queue.pop_front().unwrap();
        // Get distance to here
        let distance = *distances.get(&point).unwrap();

        let next_points = get_next_vertices(point, &edges, &vertices);

        // println!(
        //     "Expedite {:?}: {:?}; next_points: {:?}",
        //     point, distance, next_points
        // );

        for next_point in next_points {
            let current_distance = *distances.get(&next_point).unwrap();
            if current_distance > distance + 1 {
                distances.insert(next_point, distance + 1);
                point_queue.push_back(next_point);
            }
        }
    }
    // println!("Distances: {:?}", distances);

    println!("Part 1: {:?}", distances.values().max().unwrap());
    println!(
        "Part 2: {:?}",
        distances.values().filter(|x| **x >= 1000).count()
    );
}
