use std::fmt::Display;

use std::cell::RefCell;
use std::rc::Rc;
pub struct TreeNode<T: Ord + Display> {
    pub data: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Display> TreeNode<T> {
    pub fn new(_data: T) -> Self {
        TreeNode {
            data: _data,
            left: None,
            right: None
        }
    }
}


