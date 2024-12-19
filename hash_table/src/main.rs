mod hash_table;
mod key_value_pair;

use hash_table::HashTable;

fn main() {
    let mut hm: HashTable<i32, &str> = HashTable::new(7);

    hm.set(43, "hello");
    hm.set(23, "how are you");
    hm.set(12, "i am fine");
    hm.set(453, "thank you");
    hm.set(463, "how about you");
    hm.set(453, "i am fine thank you");

    hm.print();

    println!("{}", hm[43]);
    println!("{}", hm[12]);
    println!("{}", hm[463]);
    println!("{}", hm[12]);
    println!("{}", hm[23]);

    println!("{}", hm.get_size());

    hm[12] = "it is operator overloading";
    hm[23] = "i changed value of 23";

    println!("{}", hm[23]);

    hm.remove(463);

    hm.print();
}
