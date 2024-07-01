use std::cell::RefCell;
use std::rc::Rc;
struct LinkedListNode {
    data: i32,
    next: Option<Rc<RefCell<LinkedListNode>>>
}

pub struct LinkedList {
    head: Option<Rc<RefCell<LinkedListNode>>>,
    tail: Option<Rc<RefCell<LinkedListNode>>>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {head: None, tail: None}
    }

    pub fn add(&mut self, num:i32) {
        let new_node = Rc::new(RefCell::new(LinkedListNode{data: num, next: None}));
        if self.head.is_none(){
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            return;
        }
        if let Some(ref tail) = self.tail {
            tail.borrow_mut().next = Some(new_node.clone());
        }
        self.tail = Some(new_node);
    }

    pub fn remove(&mut self, num:i32) {
        let parent = self.find_parent(num);
        if let Some(node) = parent {
            let next_opt_node = node.borrow().next.clone();
            if let Some(next_node) =  next_opt_node {
                node.borrow_mut().next = next_node.borrow().next.clone();
            }
        }
    }

    fn find_parent(&mut self, num:i32) -> Option<Rc<RefCell<LinkedListNode>>> {
        let mut parent = None;
        let mut curr_node = self.head.as_ref().map(|head_ptr| Rc::clone(head_ptr));

        while let Some(ref node) = curr_node {
            let next_node = node.borrow().next.clone();
            if let Some(ref next) = next_node {
                if next.borrow().data == num {
                    parent = Some(node.clone()); 
                    break;
                }
            }else {
                break;
            }
            curr_node = next_node.as_ref().map(|next| Rc::clone(next));
        }
        parent
    }

    fn find_node(&mut self, num:i32) -> Option<Rc<RefCell<LinkedListNode>>> {
        let mut curr_node = self.head.as_ref().map(|head| Rc::clone(head));
        while let Some(ll_node) = curr_node {
            let node_borrowed = ll_node.borrow();
            if node_borrowed.data == num {
                return Some(ll_node.clone());
            }
            curr_node = node_borrowed.next.as_ref().map(|next_node| Rc::clone(next_node));
        }
        None
    }

    pub fn add_after(&mut self, parent_num:i32, num:i32) {
        let parent_node = self.find_node(parent_num);
        if let Some(ll_node) = parent_node {
            let old_next_node = ll_node.borrow_mut().next.take();
            let new_node = Some(Rc::new(RefCell::new(LinkedListNode {data: num, next: old_next_node})));
            ll_node.borrow_mut().next = new_node;
        }
    }

    pub fn add_before(&mut self, child_num:i32, num:i32) {
        let parent_node = self.find_parent(child_num);
        if let Some(ll_node) = parent_node {

            let old_next_node = ll_node.borrow_mut().next.take();
            let new_node = Some(Rc::new(RefCell::new(LinkedListNode {data: num, next: old_next_node})));
            ll_node.borrow_mut().next = new_node;
        } 
    }

    pub fn print_list(&self) {
        let mut current = self.head.as_ref().map(|head| Rc::clone(head));
        while let Some(node) = current {
            let node_ref = node.borrow();
            print!("{} -> ", node_ref.data);
            current = node_ref.next.as_ref().map(|next| Rc::clone(next));
        }
        println!();
    }

}
