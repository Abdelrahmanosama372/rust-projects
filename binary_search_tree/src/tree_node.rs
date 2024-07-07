use std::cmp::Eq;
use std::fmt::Display;

pub struct TreeNode<T: Eq + Display> {
    pub data: T,
    pub left: Box<Option<TreeNode<T>>>,
    pub right: Box<Option<TreeNode<T>>>,
}

impl<T: Eq + Display> TreeNode<T> {
    pub fn new(_data: T) -> Self {
        TreeNode {
            data: _data,
            left: Box::new(None),
            right: Box::new(None)
        }
    }
}


