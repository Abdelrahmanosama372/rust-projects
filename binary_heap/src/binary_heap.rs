use std::fmt::Display;


pub struct BinaryHeap<T: Ord> {
    nodes: Vec<T>
}

impl <T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { nodes: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.nodes.push(value);
        self.heapify_up();
    }

    fn heapify_up(&mut self) {
        let mut i = self.nodes.len() - 1;
        let mut parent_index: i32 = (i as i32 -1)/2;

        while i > 0 &&  self.nodes[parent_index as usize] > self.nodes[i] {
            self.nodes.swap(i, parent_index as usize);
            i = parent_index as usize;
            parent_index = (i as i32 -1)/2;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.len() == 0 {
            return None;
        }
        
        let data = self.nodes.swap_remove(0);
        self.heapify_down();

        return Some(data);
    }

    fn heapify_down(&mut self) {
        let mut i = 0;

        let mut left_index = 2*i + 1;
        
        while self.nodes.len() > 0 && left_index <= self.nodes.len()-1 {
            let right_index = 2*i + 2;

            let mut smaller_index = left_index;
            if right_index <= self.nodes.len() -1 && self.nodes[right_index] < self.nodes[left_index] {
                smaller_index = right_index;
            }

            if self.nodes[i] < self.nodes[smaller_index] {
                return;
            }

            self.nodes.swap(i, smaller_index);

            i = smaller_index;
            left_index = 2*i + 1;   
        }
    }

    pub fn print(&self) where T: Display
    {
       for val in self.nodes.iter() {
        print!("{} -> ",val)
       }

    }
}


#[cfg(test)]
mod tests {
    use super::BinaryHeap;

    #[test]
    fn test_insert() {
        let mut heap = BinaryHeap::new();
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);

        assert_eq!(heap.nodes, vec![1, 3, 2]);
    }

    #[test]
    fn test_pop() {
        let mut heap = BinaryHeap::new();
        heap.insert(3);
        heap.insert(1);
        heap.insert(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heapify_up() {
        let mut heap = BinaryHeap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(5);

        assert_eq!(heap.nodes, vec![5, 20, 10]);
    }

    #[test]
    fn test_heapify_down() {
        let mut heap = BinaryHeap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(5);
        heap.insert(6);
        heap.insert(1);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.nodes, vec![5, 6, 10, 20]);
    }

    #[test]
    fn test_empty_heap() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_single_element() {
        let mut heap = BinaryHeap::new();
        heap.insert(42);
        assert_eq!(heap.pop(), Some(42));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_duplicates() {
        let mut heap = BinaryHeap::new();
        heap.insert(5);
        heap.insert(5);
        heap.insert(5);

        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_large_heap() {
        let mut heap = BinaryHeap::new();
        for i in 0..1000 {
            heap.insert(i);
        }

        for i in 0..1000 {
            assert_eq!(heap.pop(), Some(i));
        }
    }
}
