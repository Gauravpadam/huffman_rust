use std::{collections::{BinaryHeap, HashMap}};

// Assuming `make_huffmap` and `Node` are implemented correctly
use huff_map::huff_map::make_huffmap;
use node::tree_node::Node;

mod huff_map;
mod node;

fn main() {
    let mut encoder: HashMap<char, String> = HashMap::new();
    let mut decoder: HashMap<String, char> = HashMap::new();

    let huff_map = make_huffmap("abccgba");

    let mut huff_heap: BinaryHeap<Node> = BinaryHeap::new();

    for (key, val) in huff_map.iter() {
        let new_node: Node = Node::new(key.clone(), *val);
        huff_heap.push(new_node);
    }

    while huff_heap.len() > 1 {
        let first = huff_heap.pop().unwrap();
        let second = huff_heap.pop().unwrap();

        let mut new_node = Node::new('\0', first.cost + second.cost);
        new_node.left = Some(Box::new(first));
        new_node.right = Some(Box::new(second));

        huff_heap.push(new_node);
    }

    let ft = huff_heap.pop();
    let string = String::new();
    
    init_maps(&mut encoder, &mut decoder, ft, &string);

    // For debugging purposes, print out the encoder and decoder maps
    println!("Encoder: {:?}", encoder);
    println!("Decoder: {:?}", decoder);
}

fn init_maps(
    encoder: &mut HashMap<char, String>, 
    decoder: &mut HashMap<String, char>, 
    node: Option<Node>, 
    string: &String
) {
    if let Some(n) = node {
        if n.left.is_none() && n.right.is_none() {
            encoder.insert(n.data, string.clone());
            decoder.insert(string.clone(), n.data);
        } else {
            // Recursively build the binary encoding
            let mut left_string = string.clone();
            left_string.push('0');
            init_maps(encoder, decoder, n.left.map(|b| *b), &left_string);

            let mut right_string = string.clone();
            right_string.push('1');
            init_maps(encoder, decoder, n.right.map(|b| *b), &right_string);
        }
    }
}
