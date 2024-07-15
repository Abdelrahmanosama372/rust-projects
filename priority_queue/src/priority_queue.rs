use std::fmt::Display;

pub struct PriorityQueueNode<T: Ord, U> {
    priority: T,
    data: U
}

pub struct PriorityQueue<T: Ord, U> {
    nodes: Vec<PriorityQueueNode<T,U>>
}

impl <T: Ord, U> PriorityQueue<T,U> {
    pub fn new() -> Self {
        PriorityQueue { nodes: Vec::new() }
    }

    pub fn insert(&mut self, priority: T, data: U) {
        let new_node = PriorityQueueNode {priority, data};
        self.nodes.push(new_node);
        self.heapify_up();
    }

    fn heapify_up(&mut self) {
        let mut i = self.nodes.len() - 1;
        let mut parent_index: i32 = (i as i32 -1)/2;

        while i > 0 &&  self.nodes[parent_index as usize].priority > self.nodes[i].priority {
            self.nodes.swap(i, parent_index as usize);
            i = parent_index as usize;
            parent_index = (i as i32 -1)/2;
        }
    }

    pub fn pop(&mut self) -> Option<(T,U)> {
        if self.nodes.len() == 0 {
            return None;
        }
        
        let node = self.nodes.swap_remove(0);
        self.heapify_down();

        return Some((node.priority,node.data));
    }

    fn heapify_down(&mut self) {

        if self.nodes.len() == 0 {
            return;
        }

        let mut i = 0;

        let mut left_index = 2*i + 1;
        
        while left_index <= self.nodes.len()-1 {
            let right_index = 2*i + 2;

            let mut smaller_index = left_index;
            if right_index <= self.nodes.len() -1 && self.nodes[right_index].priority < self.nodes[left_index].priority {
                smaller_index = right_index;
            }

            if self.nodes[i].priority < self.nodes[smaller_index].priority {
                return;
            }

            self.nodes.swap(i, smaller_index);

            i = smaller_index;
            left_index = 2*i + 1;   
        }
    }

    pub fn print(&self) where T: Display, U: Display
    {
       for node in self.nodes.iter() {
        println!("[{}]{}-> ",node.priority,node.data)
       }

    }
}