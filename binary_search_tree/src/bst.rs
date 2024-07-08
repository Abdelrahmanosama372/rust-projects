
use crate::tree_node::TreeNode;
use std::fmt::Display;

pub struct TreeNodeWithParent<'a,T: Ord + Display> {
    pub node: &'a TreeNode<T>,
    pub parent: &'a Option<TreeNode<T>>,
    pub is_left: bool,
}
pub struct BST<T: Ord + Display + Clone> {
    root: Option<TreeNode<T>>
}

impl<T: Ord + Display + Clone> BST<T> {

    pub fn new() -> Self {
        BST {root: None}
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Some(TreeNode::new(data.clone()));
        if self.root.is_none() {
            self.root = new_node;
            return;
        }
        
        let mut curr_node = &mut self.root;
        while !curr_node.is_none() {
            if let Some(ref mut node_content) = curr_node {
                if node_content.data > data {
                    if (&*node_content.left).is_none() {
                        node_content.left = Box::new(new_node);
                        break;
                    }else {
                        curr_node = &mut *node_content.left;
                    }
                }else {
                    if (&*node_content.right).is_none() {
                        node_content.right = Box::new(new_node);
                        break;
                    }else {
                        curr_node = &mut *node_content.right;
                    }
                }
            }
        }
    }

    pub fn find(&self, data: T) -> Option<&TreeNode<T>>{
        
        let mut curr_node = &self.root;
        while !curr_node.is_none() {
            if let Some(ref node_content) = curr_node {
                if node_content.data == data {
                    return Some(node_content);
                }else if node_content.data > data {
                    curr_node = &*node_content.left;
                }else {
                    curr_node = &*node_content.right;
                }
            }
        }
        None
    }

    pub fn find_parent(&self, data: T) -> Option<TreeNodeWithParent<T>>{  
        let mut curr_node = &self.root;
        let mut parent_node = &None;
        let mut _is_left = true;

        while !curr_node.is_none() {
            if let Some(ref node_content) = curr_node {
                if node_content.data == data {
                    return Some(
                        TreeNodeWithParent {
                            node: node_content,
                            parent: parent_node,
                            is_left: _is_left
                        }
                    );
                }else if node_content.data > data {
                    parent_node = curr_node;
                    curr_node = &*node_content.left;
                    _is_left = true;
                }else {
                    parent_node = curr_node;
                    curr_node = &*node_content.right;
                    _is_left = false;
                }
            }
        }

        None
    }


    pub fn height(&self) -> u32 {
        return  self.internal_height(&self.root);
    }

    fn internal_height(&self, node: &Option<TreeNode<T>>) -> u32 {
        if let Some(node_content) = node {
            return 1 + std::cmp::max(self.internal_height(&*node_content.left), self.internal_height(&*node_content.right));
        }else {
            return 0;
        }
        
    }

    pub fn pre_order_traversal(&self) {
        self.pre_order_traversal_internal(&self.root);
    }

    fn pre_order_traversal_internal(&self, node: &Option<TreeNode<T>>) {
        if let Some(node_content) = node {
            print!("{} -> ",node_content.data);
            self.pre_order_traversal_internal(&*node_content.left);
            self.pre_order_traversal_internal(&*node_content.right);
        }
    }

    pub fn in_order_traversal(&self) {
        self.in_order_traversal_internal(&self.root);
    }

    fn in_order_traversal_internal(&self, node: &Option<TreeNode<T>>) {
        if let Some(node_content) = node {
            self.in_order_traversal_internal(&*node_content.left);
            print!("{} -> ",node_content.data);
            self.in_order_traversal_internal(&*node_content.right);
        }
    }

    pub fn post_order_traversal(&self) {
        self.post_order_traversal_internal(&self.root);
    }

    fn post_order_traversal_internal(&self, node: &Option<TreeNode<T>>) {
        if let Some(node_content) = node {
            self.post_order_traversal_internal(&*node_content.left);
            self.post_order_traversal_internal(&*node_content.right);
            print!("{} -> ",node_content.data);
        }
    }


}