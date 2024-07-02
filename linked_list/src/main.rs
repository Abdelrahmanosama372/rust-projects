use linked_list::LinkedList;

fn main() {
   
    let mut ll = LinkedList::new();
    ll.insert_last(1);
    ll.insert_last(2);
    ll.insert_last(3);
    ll.insert_last(4);
    ll.insert_last(5);
    ll.insert_last(6);
    ll.insert_last(7);
    ll.insert_last(8);
    ll.insert_last(9);
    ll.insert_last(10);
    ll.print_list();

    ll.remove(7);
    ll.print_list();

    ll.insert_before(5,40);
    ll.insert_before(1,30);
    ll.insert_after(10, 11);
    ll.insert_after(40, 21);
    ll.remove(30);
    ll.remove(1);
    ll.print_list();
}
