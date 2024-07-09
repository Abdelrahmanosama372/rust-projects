use crate::tree_node::TreeNode;
use std::{cell::RefCell, fmt::Display, rc::Rc};

pub struct TreeNodeWithParent<T: Ord + Display> {
    pub node: Option<Rc<RefCell<TreeNode<T>>>>,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub is_left: bool,
}

pub struct BST<T: Ord + Display + Clone> {
    root: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T: Ord + Display + Clone> BST<T> {

    pub fn new() -> Self {
        BST {root: None}
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(data.clone())));
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }
        
        let mut curr_node = self.root.as_ref().map(|h| Rc::clone(h));
        
        while let Some(node_content) = curr_node {
            if node_content.borrow().data > data {
                if (node_content.borrow().left).is_none() {
                    node_content.borrow_mut().left = Some(new_node);
                    break;
                }else {
                    curr_node = node_content.borrow().left.as_ref().map(|h| Rc::clone(h));
                }
            }else {
                if node_content.borrow().right.is_none() {
                    node_content.borrow_mut().right = Some(new_node);
                    break;
                }else {
                    curr_node = node_content.borrow().right.as_ref().map(|h| Rc::clone(h));
                }
            }
        }
    }

    pub fn find(&self, data: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        
        let mut curr_node = self.root.as_ref().map(|f| Rc::clone(f));
      
        while let Some(node_content) = curr_node {
            if node_content.borrow().data == data {
                return Some(Rc::clone(&node_content));
            }else if node_content.borrow().data > data {
                curr_node = node_content.borrow().left.as_ref().map(|h| Rc::clone(h));
            }else {
                curr_node = node_content.borrow().right.as_ref().map(|h| Rc::clone(h));
            }
        }
        None
    }
    
    pub fn find_parent(&mut self, data: T) -> Option<TreeNodeWithParent<T>> {  

        let mut curr_node = self.root.as_ref().map(|h| Rc::clone(h));
        let mut parent = None;
        let mut _is_left = false;

        while let Some(node) =  curr_node {
            if node.borrow().data == data {
                return Some(TreeNodeWithParent {
                    node: Some(node),
                    parent: parent,
                    is_left: _is_left
                });
            }else if node.borrow().data > data {
                curr_node = node.borrow().left.as_ref().map(|h| Rc::clone(h));
                _is_left = true;
            }else {
                curr_node = node.borrow().right.as_ref().map(|h| Rc::clone(h));
                _is_left = false;
            }
            parent = Some(Rc::clone(&node));
        }
        None
    }


    pub fn height(&self) -> u32 {
        return  self.internal_height(&self.root);
    }

    fn internal_height(&self, node: &Option<Rc<RefCell<TreeNode<T>>>>) -> u32 {
        if let Some(node_content) = node {
            return 1 + std::cmp::max(self.internal_height(&node_content.borrow().left), self.internal_height(&node_content.borrow().right));
        }else {
            return 0;
        }
        
    }

    pub fn pre_order_traversal(&self) {
        self.pre_order_traversal_internal(&self.root);
    }

    fn pre_order_traversal_internal(&self, node: &Option<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(node_content) = node {
            print!("{} -> ",node_content.borrow().data);
            self.pre_order_traversal_internal(&node_content.borrow().left);
            self.pre_order_traversal_internal(&node_content.borrow().right);
        }
    }

    pub fn in_order_traversal(&self) {
        self.in_order_traversal_internal(&self.root);
    }

    fn in_order_traversal_internal(&self, node: &Option<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(node_content) = node {
            self.in_order_traversal_internal(&node_content.borrow().left);
            print!("{} -> ",node_content.borrow().data);
            self.in_order_traversal_internal(&node_content.borrow().right);
        }
    }

    pub fn post_order_traversal(&self) {
        self.post_order_traversal_internal(&self.root);
    }

    fn post_order_traversal_internal(&self, node: &Option<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(node_content) = node {
            self.post_order_traversal_internal(&node_content.borrow().left);
            self.post_order_traversal_internal(&node_content.borrow().right);
            print!("{} -> ",node_content.borrow().data);
        }
    }


}