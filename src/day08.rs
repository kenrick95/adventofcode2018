// use std::io;
use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
	data: Vec<i32>,
	child: Vec<Node>,
}

fn construct_tree(list: &mut VecDeque<i32>) -> Node {
	let mut data: Vec<i32> = vec![];
	let mut child: Vec<Node> = vec![];

	let num_children = list.pop_front().unwrap();
	let num_data = list.pop_front().unwrap();
	// println!("{:?}", num_children);


	// Recurse to construct children
	{
		let mut i = 0;
		while i < num_children {
			child.push(construct_tree(list));
			i += 1;
		}
	}

	// Construct data
	{
		let mut i = 0;
		while i < num_data {
			data.push(list.pop_front().unwrap());
			i += 1;
		}
	}

	return Node {
		data,
		child
	}
}

fn sum_data(node: &Node) -> i32 {
	return node.data
			.iter()
			.sum::<i32>()
		+ node.child
			.iter()
			.map(|c| sum_data(c))
			.sum::<i32>();
}

fn get_values(node: &Node) -> i32 {
	// If node does not have child, return sum of node.data
	if node.child.len() == 0 {
		return node.data.iter().sum::<i32>();
	}
	// else return sum of node.data.map(datum => node.child[datum])
	// Note: 1-indexed, 1 --> child[0]
	// Note 2: 0 --> 0
	// Note 3: if index nodes not exist --> 0

	return node.data
		.iter()
		.map(|d| {
			if *d > 0 && *d as usize <= node.child.len() {
				return get_values(&node.child[*d as usize - 1]);
			}
			return 0;
		})
		.sum::<i32>()
}

pub fn main() {
	// let mut input = String::new();
    let mut input =
        String::from(fs::read_to_string("./src/day08.in").expect("Unable to read file"));
    // io::stdin().read_line(&mut input).unwrap();
    input = String::from(input.trim());

    // Split string based on space
    // Dequeue, so I could pop_front efficiently...
    let mut v: VecDeque<i32> = input
    	.split_whitespace()
    	.map(|s| String::from(s).parse().unwrap())
    	.collect();

    let tree = construct_tree(&mut v);
    // println!("Tree: {:?}", tree);

    println!("Part 1: {:?}", sum_data(&tree));
    println!("Part 2: {:?}", get_values(&tree));

}