use std::io;

// Note: "pointer" is an index in List.nodes
#[derive(Debug)]
struct Node {
    value: usize,

    next_pointer: usize,
    prev_pointer: usize,
}

// Circular doubly linked list
// Since it's circular, there is no representation of "null" pointer
// When an element is removed from list, that node isn't actually removed from the Vec, leaving some memory footprint behind
#[derive(Debug)]
struct List {
    nodes: Vec<Node>,

    current_pointer: usize,
}

impl List {
    fn insert(&mut self, pointer: usize, value: usize) -> usize {
        let current_next_pointer = self.nodes[pointer].next_pointer;

        self.nodes.push(Node {
            value: value,
            next_pointer: current_next_pointer,
            prev_pointer: pointer,
        });
        let new_pointer = self.nodes.len() - 1;
        self.nodes[pointer].next_pointer = new_pointer;
        self.nodes[current_next_pointer].prev_pointer = new_pointer;

        return new_pointer;
    }

    fn remove(&mut self, pointer: usize) -> usize {
        let current_next_pointer = self.nodes[pointer].next_pointer;
        let current_prev_pointer = self.nodes[pointer].prev_pointer;
        self.nodes[current_prev_pointer].next_pointer = current_next_pointer;
        self.nodes[current_next_pointer].prev_pointer = current_prev_pointer;
        return self.nodes[pointer].value;
    }

    fn next(&mut self) -> usize {
        let next_pointer = self.nodes[self.current_pointer].next_pointer;
        self.current_pointer = next_pointer;
        return next_pointer;
    }
    fn prev(&mut self) -> usize {
        let prev_pointer = self.nodes[self.current_pointer].prev_pointer;
        self.current_pointer = prev_pointer;
        return prev_pointer;
    }

    fn forward(&mut self, count: usize) -> usize {
        let mut i = 0;
        let mut final_pointer = 0;
        while i < count {
            final_pointer = self.next();
            i += 1;
        }
        return final_pointer;
    }
    fn backward(&mut self, count: usize) -> usize {
        let mut i = 0;
        let mut final_pointer = 0;
        while i < count {
            final_pointer = self.prev();
            i += 1;
        }
        return final_pointer;
    }
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = String::from(input.trim());
    let num_players: usize = input.parse().unwrap();

    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = String::from(input.trim());
    let last_marble: usize = input.parse().unwrap();

    let mut player_scores: Vec<usize> = Vec::new();
    player_scores.resize(num_players, 0);

    // let mut marbles: Vec<usize> = vec![0];
    let mut current_marble_external: usize = 0;
    let mut current_player: usize = 0;
    // let mut cm_index: usize = 0;
    // let mut current_marble: usize = 0;

    let mut list = List {
        nodes: vec![Node {
            value: 0,
            next_pointer: 0,
            prev_pointer: 0,
        }],
        current_pointer: 0,
    };

    // println!(
    //     "num_players {:?}, last_marble {:?}",
    //     num_players, last_marble,
    // );

    while current_marble_external < last_marble {
        current_marble_external += 1;
        if current_marble_external % 23 == 0 {
            player_scores[current_player] += current_marble_external;
            let pointer = list.backward(7);
            // println!(
            //     "pointer removed {}, value {:?}",
            //     pointer, list.nodes[pointer]
            // );
            player_scores[current_player] += list.remove(pointer);
            list.current_pointer = list.nodes[pointer].next_pointer;

        // println!(
        //     "Player {:?} new score is {:?}",
        //     current_player, player_scores[current_player],
        // );
        } else {
            let pointer = list.forward(1);
            list.current_pointer = list.insert(pointer, current_marble_external);
        }
        // println!(
        //     "cme: {}, current_pointer {}, value {:?}",
        //     current_marble_external, list.current_pointer, list.nodes[list.current_pointer]
        // );

        // println!(
        //     "[{:?}] current_marble {:?}, cme {:?}, cm_index {:?}, marbles {:?}",
        //     current_player, current_marble, current_marble_external, cm_index, marbles
        // );
        // println!("[{:?}], {:?}, list {:?}", current_player, list.current_pointer, list);

        current_player = (current_player + 1) % num_players;
    }
    // println!("{:?}", list);

    // println!("{:?}", player_scores);
    println!("Part 1: {:?}", player_scores.iter().max().unwrap());
}
