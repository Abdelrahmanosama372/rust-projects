use linked_list::LinkedList;

fn main() {
   
    let mut ll = LinkedList::new();
    ll.add(1);
    ll.add(2);
    ll.add(3);
    ll.add(4);
    ll.add(5);
    ll.add(6);
    ll.add(7);
    ll.add(8);
    ll.add(9);
    ll.add(10);
    ll.print_list();

    ll.remove(7);
    ll.print_list();

    ll.add_before(5,40);
    ll.add_after(10, 11);
    ll.add_after(5, 20);
    ll.print_list();
}
