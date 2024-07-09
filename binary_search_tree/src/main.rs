pub mod tree_node;
pub mod bst;

use bst::BST;

fn main() {
    let mut bst: BST<i32> = BST::new();
   
    bst.insert(8);
    bst.insert(3);
    bst.insert(10);
    bst.insert(1);
    bst.insert(6);
    bst.insert(14);
    bst.insert(4);
    bst.insert(7);
    bst.insert(13);

    bst.pre_order_traversal();
    println!();
    bst.in_order_traversal();
    println!();
    bst.post_order_traversal();
    println!();

    let h = bst.find_parent(3).unwrap();
    let node = h.node.unwrap();
    let parent = h.parent.unwrap();

    println!("child: {} parent: {} is_left: {}",node.borrow().data,parent.borrow().data,h.is_left);


    bst.in_order_traversal();
    println!();
    
}
