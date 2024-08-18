use core::hash::Hash;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Node {
    data: char,
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: char, freq: u32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            data,
            freq,
            left,
            right,
        }
    }
}

struct Huffman {
    pq: PriorityQueue<Node, Reverse<u32>>,
    codes: HashMap<char, String>,
}

impl Huffman {
    fn new(string: &str) -> Self {
        let mut frequencies: HashMap<char, u32> = HashMap::new();
        for ch in string.chars() {
            if frequencies.contains_key(&ch) {
                let val = frequencies.get_mut(&ch).unwrap();
                (*val) += 1;
            } else {
                frequencies.insert(ch, 1);
            }
        }

        let mut pq = PriorityQueue::new();
        for (ch, freq) in frequencies.into_iter() {
            let node = Node::new(ch, freq, None, None);
            pq.push(node, Reverse(freq));
        }

        while pq.len() != 1 {
            let (left_node, _) = pq.pop().unwrap();
            let (right_node, _) = pq.pop().unwrap();
            let new_node = Node::new(
                ' ',
                left_node.freq + right_node.freq,
                Some(Box::new(left_node)),
                Some(Box::new(right_node)),
            );

            let freq = new_node.freq;
            pq.push(new_node, Reverse(freq));
        }

        Self {
            pq,
            codes: HashMap::new(),
        }
    }

    pub fn generate_codes(&mut self) {
        let node = self.pq.pop().unwrap().0;
        self.generate_codes_internal(&node, "");
    }

    fn generate_codes_internal(&mut self, node: &Node, code: &str) {
        if node.left.is_none() && node.right.is_none() {
            self.codes.insert(node.data, code.to_string());
        }

        if let Some(left_node) = &node.left {
            self.generate_codes_internal(&left_node, &(code.to_string() + "0"));
        }

        if let Some(right_node) = &node.right {
            self.generate_codes_internal(&right_node, &(code.to_string() + "1"));
        }
    }
}

fn main() {
    let mut h = Huffman::new("internet");
    h.generate_codes();

    for (ch, code) in h.codes.into_iter() {
        println!("{} {}", ch, code);
    }
}
