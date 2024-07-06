mod stack;
use stack::Stack;

fn main() {
    let mut stack = Stack::new();
    
    println!("Pushing elements onto the stack:");
    for i in 1..=5 {
        println!("Push: {}", i);
        stack.push(i);
    }

    println!("\nCurrent top element: {:?}", stack.top().unwrap());
    println!("Current stack size: {}", stack.get_size());

    println!("\nPopping elements from the stack:");
    while !stack.is_empty() {
        println!("Pop: {:?}", stack.pop().unwrap());
    }

    println!("\nIs the stack empty? {}", stack.is_empty());
}
