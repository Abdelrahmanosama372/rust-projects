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

    bst.in_order_traversal();
    println!();

    let t = bst.find_parent(3).unwrap();

    if let Some(j) = t.parent {
        println!("parent: {}, child: {}, isleft: {}",j.data ,t.node.data, t.is_left)
    }
   

}
