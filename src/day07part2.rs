use regex::Regex;
use std::collections::HashSet;
use std::io;

#[path = "./day07part1.rs"]
mod day07part1;

const MAX_NODES: usize = 26;
const MAX_WORKERS: usize = 5;
const MODIFIER: usize = 60;

fn get_node_weight(node: usize) -> usize {
    return node + 1 + MODIFIER;
}

pub fn main() {
    let mut edges = day07part1::get_edges();
    let mut time_counter: usize = 0;
    let mut worker_next_free_time = Vec::new();
    let mut worker_current_task: Vec<i32> = Vec::new();

    {
        let mut i = 0;
        while i < MAX_WORKERS {
            worker_next_free_time.push(0);
            worker_current_task.push(-1);
            i += 1;
        }
    }

    {
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

        loop {
            // Assign tasks to workers, until either all workers are busy or no task is available

            let mut is_all_workers_free: bool = true;
            loop {
                let mut worker_index: i32 = -1;
                {
                    let mut i = 0;
                    while i < MAX_WORKERS {
                        if worker_next_free_time[i] <= time_counter {
                            if worker_index == -1 {
                                worker_index = i as i32;
                            }
                        } else {
                            is_all_workers_free = false;
                        }
                        i += 1;
                    }
                }
                if is_all_workers_free && S.is_empty() {
                    break;
                }
                // println!("{}, worker_index {:?}", time_counter, worker_index);
                if worker_index == -1 {
                    break;
                }

                if worker_index > -1 {
                    // Get next available node from S
                    if S.is_empty() {
                        // no available task... continue working
                        break;
                    } else {
                        // a task is available for worker with index "worker_index"
                        let node = *(S.iter().min().unwrap());
                        println!(
                            "{}, worker_index {:?} node {:?}",
                            time_counter, worker_index, node
                        );
                        S.remove(&node);
                        worker_next_free_time[worker_index as usize] =
                            time_counter + get_node_weight(node);
                        worker_current_task[worker_index as usize] = node as i32;
                    }
                }
            }
            if is_all_workers_free && S.is_empty() {
                break;
            }

            // println!(
            //     "{}, worker_next_free_time {:?}",
            //     time_counter, worker_next_free_time
            // );
            // println!(
            //     "{}, worker_current_task {:?}",
            //     time_counter, worker_current_task
            // );

            // If worker has finished, add task to S
            {
                let mut i = 0;
                while i < MAX_WORKERS {
                    if worker_next_free_time[i] == time_counter + 1 && worker_current_task[i] > -1 {
                        let node = worker_current_task[i] as usize;
                        // println!("{}, node {:?}", time_counter, node);
                        let mut node_m = 0;
                        // for each node m with an edge e from n to m do
                        while node_m < MAX_NODES {
                            if edges[node][node_m] {
                                // remove edge e from the graph
                                edges[node][node_m] = false;

                                // if m has no other incoming edges then

                                // insert m into S
                                let mut j = 0;
                                let mut okay = true;
                                while j < MAX_NODES {
                                    if edges[j][node_m] {
                                        // There is an incoming edge from j to node_m
                                        okay = false;
                                        break;
                                    }
                                    j += 1;
                                }
                                if okay {
                                    S.insert(node_m);
                                }
                            }
                            node_m += 1;
                        }
                    }

                    i += 1;
                }
            }

            time_counter += 1;
            println!("{}, S: {:?}", time_counter, S);
        }
    }
    println!("Part 2: {:?}", time_counter);
}
