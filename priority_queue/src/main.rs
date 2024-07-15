
pub mod priority_queue;

use priority_queue::PriorityQueue;

fn main() {
    let mut pq = PriorityQueue::new();

    pq.insert(5, 56);
    pq.insert(5, 93);
    pq.insert(3, 28);
    pq.insert(4, 30);
    pq.insert(2, 50);
    pq.insert(1, 20);
    pq.insert(1, 58);
    pq.insert(2, 22);
    pq.insert(3, 12);

    for _ in 0..9 {
        let node = pq.pop().unwrap();
        println!("[{}]{}",node.0,node.1);
    }

    // pq.print();
}
