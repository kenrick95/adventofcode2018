use regex::Regex;
use std::collections::HashSet;
use std::io;

const MAX_NODES: usize = 26;
pub fn get_edges() -> Vec<Vec<bool>> {
    let mut edges: Vec<Vec<bool>> = Vec::new();
    {
        let mut i = 0;
        while i < MAX_NODES {
            let mut j = 0;
            let mut tmp = Vec::new();
            while j < MAX_NODES {
                tmp.push(false);
                j += 1;
            }
            edges.push(tmp);
            // edges[i][i] = false;
            i += 1;
        }
    }

    let re = Regex::new(r"Step (.) must be finished before step (.) can begin\.").unwrap();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }

        let caps = re.captures(input.as_str()).unwrap();
        let step_before = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let step_after = caps.get(2).unwrap().as_str().chars().next().unwrap();

        // println!("{:?}->{:?}", step_before, step_after);
        // NOTE: u8 is to work with ascii characters (because "char" does not have subtraction trait)
        let step_before_index = ((step_before as u8) - ('A' as u8)) as usize;
        let step_after_index = ((step_after as u8) - ('A' as u8)) as usize;

        edges[step_before_index][step_after_index] = true;
    }
    return edges;
}

pub fn main() {
    let mut edges = get_edges();

    let mut ans: Vec<char> = Vec::new();

    // Topo sort
    // [Kahn's algorithm](https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm)
    {
        let mut L: Vec<usize> = Vec::new();
        let mut S: HashSet<usize> = HashSet::new();

        // insert nodes without incoming edge to S
        {
            let mut i = 0;
            while i < MAX_NODES {
                // Check whether node i can be added to S
                let mut j = 0;
                let mut okay = true;
                while j < MAX_NODES {
                    if edges[j][i] {
                        // There is an incoming edge from j to i
                        okay = false;
                        break;
                    }
                    j += 1;
                }
                if okay {
                    S.insert(i);
                }
                i += 1;
            }
            // println!("S: {:?}", S);
        }

        while !S.is_empty() {
            // println!("S: {:?}", S);
            // remove a node n from S
            let node = *(S.iter().min().unwrap());
            S.remove(&node);

            // add n to tail of L
            L.push(node);
            {
                let mut i = 0;
                // for each node m with an edge e from n to m do
                while i < MAX_NODES {
                    if edges[node][i] {
                        // remove edge e from the graph
                        edges[node][i] = false;

                        // if m has no other incoming edges then

                        // insert m into S
                        let mut j = 0;
                        let mut okay = true;
                        while j < MAX_NODES {
                            if edges[j][i] {
                                // There is an incoming edge from j to i
                                okay = false;
                                break;
                            }
                            j += 1;
                        }
                        if okay {
                            S.insert(i);
                        }
                    }
                    i += 1;
                }
            }
        }

        // println!("{:?}", S);
        // println!("{:?}", L);

        for l in L {
            ans.push(((l as u8) + ('A' as u8)) as char);
        }
    }

    let ans_string: String = ans.into_iter().collect();
    println!("Part 1: {:?}", ans_string);
}
