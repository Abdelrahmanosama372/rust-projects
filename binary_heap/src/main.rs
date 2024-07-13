pub mod binary_heap;

use binary_heap::BinaryHeap;

fn main() {
    let mut bh = BinaryHeap::new();
    bh.insert(31);
    bh.insert(21);
    bh.insert(45);
    bh.insert(12);
    bh.insert(3);
    bh.insert(6);
    bh.insert(36);

    println!("{}",bh.pop().unwrap());
    bh.print();
}
