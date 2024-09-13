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

    pub fn new(data: T) -> Self {
        BST {root: Some(Rc::new(RefCell::new(TreeNode::new(data))))}
    }

    pub fn insert(&self, data: T) {
        if let Some(ref root_node) = self.root {
            self.insert_recursive(&mut *(root_node.borrow_mut()), data);
        }
    }
    

    fn insert_recursive(&self, node: &mut TreeNode<T>, data: T)
    {
        if node.data > data {
            if node.left.is_none() {
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(data))));            
            }else {
                if let Some(ref mut left_node) = node.left {
                    self.insert_recursive(&mut *(left_node.borrow_mut()), data);
                }
            }
        }else {
            if node.right.is_none() {
                node.right = Some(Rc::new(RefCell::new(TreeNode::new(data))));
            }else {
                if let Some(ref mut right_node) = node.right {
                    self.insert_recursive(&mut *(right_node.borrow_mut()), data);
                }
            }
        }
    }


    pub  fn delete(&mut self, data: T) {
        let node_with_parent = self.find_parent(data);
        
        if let Some(node_with_parent_content) = node_with_parent {
            if let Some(node_content) = node_with_parent_content.node {
               if node_content.borrow().left.is_none() && node_content.borrow().right.is_none()  {
                    self.delete_leaf((node_with_parent_content.parent, node_with_parent_content.is_left));
               }
               else if node_content.borrow().left.is_none() ||  node_content.borrow().right.is_none() {
                    self.delete_node_with_one_child(node_content);
               }else {
                    self.delete_node_with_two_child(node_content);
               }
            }
        }
    }

    pub  fn delete_leaf(&mut self, node_tup: (Option<Rc<RefCell<TreeNode<T>>>>, bool)) {
        // delete root
        if node_tup.0.is_none() {
            self.root = None;
        }

        let parent = node_tup.0.unwrap();

        if node_tup.1 == true {
            parent.borrow_mut().left = None;
        }else {
            parent.borrow_mut().right = None;
        }
    }

    pub fn delete_node_with_one_child(&mut self, node: Rc<RefCell<TreeNode<T>>> ) {
        
        let child_node;

        if node.borrow().left.is_some() {
            child_node = node.borrow_mut().left.take().unwrap();
        }else {
            child_node = node.borrow_mut().right.take().unwrap();
        }

        node.borrow_mut().data = child_node.borrow_mut().data.clone();
        node.borrow_mut().left = child_node.borrow_mut().left.take();
        node.borrow_mut().right = child_node.borrow_mut().right.take();
    }

    pub fn delete_node_with_two_child(&mut self, node: Rc<RefCell<TreeNode<T>>> ) {
        
        let mut parent = None;
        let mut curr_node = node.borrow().right.as_ref().map(|h| Rc::clone(h)).unwrap();
        
        let mut has_left = curr_node.borrow().left.is_some();

        while has_left {
            parent = Some(Rc::clone(&curr_node));
            let node_left = curr_node.borrow().left.as_ref().map(|h| Rc::clone(h)).unwrap();
            curr_node = node_left;
            has_left = curr_node.borrow().left.is_some();
        }
        
        if parent.is_none() {
            node.borrow_mut().data = curr_node.borrow_mut().data.clone();
            node.borrow_mut().right = curr_node.borrow_mut().right.take();
        }else {
            node.borrow_mut().data = curr_node.borrow_mut().data.clone();
            if let Some(node_right) = curr_node.borrow_mut().right.take() {
                parent.unwrap().borrow_mut().left = Some(node_right);
            }else {
                parent.unwrap().borrow_mut().left = None;
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