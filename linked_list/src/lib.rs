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

    pub fn insert_last(&mut self, num:i32) {
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
            let old_next = node.borrow_mut().next.take();
            if let Some(next_node) =  old_next {
                node.borrow_mut().next = next_node.borrow_mut().next.take();
            }
        }else {
            if self.is_head(num) {
                // remove the head of the linked list
                let curr_node = self.head.as_ref().map(|head_ptr| Rc::clone(head_ptr));
                if let Some(node) = curr_node {
                    self.head = node.borrow_mut().next.take();
                }
            }
        }
    }

    fn find_parent(&mut self, num:i32) -> Option<Rc<RefCell<LinkedListNode>>> {

        let mut curr_node = self.head.as_ref().map(|head_ptr| Rc::clone(head_ptr));

        if self.is_head(num) {
            return None;
        }

        while let Some(ref node) = curr_node {
            let next_node = node.borrow().next.as_ref().map(|node| Rc::clone(node));
            if let Some(ref next) = next_node {
                if next.borrow().data == num {
                    return Some(node.clone()); 
                }
            }else {
                break;
            }
            curr_node = next_node.as_ref().map(|next| Rc::clone(next));
        }
        return None;
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

    fn is_head(&self, num: i32) -> bool {
        if let Some(ref node) = self.head {
            if node.borrow().data == num {
                return true;
            }
        }
        false
    }

    fn is_tail(&self, num: i32) -> bool {
        if let Some(ref node) = self.tail {
            if node.borrow().data == num {
                return true;
            }
        }
        false
    }

    pub fn insert_after(&mut self, parent_num:i32, num:i32) {
        if self.is_tail(parent_num) {
            self.insert_last(num);
            return;
        }

        let parent_node = self.find_node(parent_num);
        if let Some(ll_node) = parent_node {
            let old_next_node = ll_node.borrow_mut().next.take();
            let new_node = Some(Rc::new(RefCell::new(LinkedListNode {data: num, next: old_next_node})));
            ll_node.borrow_mut().next = new_node;
        }
    }

    pub fn insert_before(&mut self, child_num:i32, num:i32) {
        let parent_node = self.find_parent(child_num);
        if let Some(ll_node) = parent_node {
            let old_next_node = ll_node.borrow_mut().next.take();
            let new_node = Some(Rc::new(RefCell::new(LinkedListNode {data: num, next: old_next_node})));
            ll_node.borrow_mut().next = new_node;
        }else {
            if self.is_head(child_num) {
                // add before head
                let new_node = Some(Rc::new(RefCell::new(LinkedListNode {data: num, next: self.head.take()})));
                self.head = new_node;
            }
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

    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<i32> {
        let mut current = self.head.as_ref().map(|head| Rc::clone(head));
        let mut vec = Vec::new();
        while let Some(node) = current {
            let node_ref = node.borrow();
            vec.push(node_ref.data);
            current = node_ref.next.as_ref().map(|next| Rc::clone(next));
        }
        vec
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_last_test(){
        let mut list = LinkedList::new();
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);
        list.insert_last(4);
        list.insert_last(5);
        assert_eq!(list.to_vec(),vec![1,2,3,4,5]);
    }

    #[test]
    fn insert_before_test(){
        let mut list = LinkedList::new();
        list.insert_last(3);
        list.insert_last(5);

        list.insert_before(3,2);
        list.insert_before(2,1);
        list.insert_before(5,4);
        
        assert_eq!(list.to_vec(),vec![1,2,3,4,5]);
    }

    #[test]
    fn insert_after_test(){
        let mut list = LinkedList::new();
        list.insert_last(1);
        list.insert_last(4);

        list.insert_after(1,2);
        list.insert_after(2,3);
        list.insert_after(4,5);
        
        assert_eq!(list.to_vec(),vec![1,2,3,4,5]);
    }

    #[test]
    fn remove_test(){
        let mut list = LinkedList::new();
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);
        list.insert_last(4);
        list.insert_last(5);

        list.remove(3);
        list.remove(1); // remove head of list

        assert_eq!(list.to_vec(),vec![2,4,5]);
    }

    #[test]
    fn all_list_functions_test(){
        let mut list = LinkedList::new();
        list.insert_last(1);
        list.insert_last(2);
        list.insert_last(3);
        list.insert_last(4);
        list.insert_last(5);

        list.remove(3);
        list.remove(1); // remove head of list

        list.insert_after(2, 3);
        list.insert_before(2, 1); // insert at head of list
        
        assert_eq!(list.to_vec(),vec![1,2,3,4,5]);
    }
}