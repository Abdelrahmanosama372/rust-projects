use linked_list::LinkedList;

pub struct Stack {
    list: LinkedList,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            list: LinkedList::new()
        }
    }

    pub fn push(&mut self, num: i32) {
        self.list.insert_head(num);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let data = self.list.get_head_data();
        if let Some(num) = data {
            self.list.remove(num);
        } 
        data
    }

    pub fn top(&self) -> Option<i32> {
        self.list.get_head_data()
    }

    pub fn get_size(&self) -> usize {
        self.list.get_length()
    }

    pub fn is_empty(&self) -> bool {
        return self.list.get_length() == 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_test() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        let mut i = 5;
        while !stack.is_empty() {
            assert_eq!(stack.top(), Some(i));
            assert_eq!(stack.get_size(), i as usize);
            assert_eq!(stack.pop(), Some(i));
            i -= 1;
        }
    }

    #[test]
    fn stack_empty_pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn stack_empty_top() {
        let stack = Stack::new();
        assert_eq!(stack.top(), None);
    }
}