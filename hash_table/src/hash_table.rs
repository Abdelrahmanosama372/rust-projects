
use std::{fmt::Display, ops::{Index, IndexMut}};

use crate::key_value_pair::KeyValuePair;

pub struct HashTable <T: Clone + Ord + Display, U: Clone + Ord + Display> {
    data: Box<[Option<KeyValuePair<T, U>>]>,
    capacity : usize,
    data_count: usize
}

impl <T: Clone + Ord + Display, U: Clone + Ord + Display> HashTable<T, U> {
    
    pub fn new(initial_capacity: usize) -> Self {
        HashTable {
            data: vec![None; initial_capacity].into_boxed_slice(), 
            capacity: initial_capacity,
            data_count: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.data_count
    }

    fn get_hash(&self, key: &T) -> usize {
        let offset_basis: u32 = 2166136261;
        let fnv_prime: u32 = 16777619;

        let mut hash = offset_basis;

        for byte in key.to_string().as_bytes() {
            hash ^= *byte as u32;
            hash = hash.wrapping_mul(fnv_prime);
        }

        return (hash % self.capacity as u32) as usize;
    }

    fn resize_or_not(&mut self) {
        if self.data_count < self.capacity {
            return;
        }

        println!("i am doing resizing");

        let mut new_data = self.data.clone();
        self.data = vec![None; self.capacity * 2].into_boxed_slice();
        self.data_count = 0;
        self.capacity *= 2;

        for i in 0..new_data.len() {

            if let Some(entry) = new_data[i].take() {
                self.add_entry(entry.key, entry.value);
            }
        }
    }

    fn collision_handling(&self, key: &T, hash: usize, set: bool) -> Option<usize> {
        for i in 1..self.capacity {
            let new_hash = (hash + i) % self.capacity;
            if let Some(ref entry) = self.data[new_hash] {
                if entry.key == *key {  
                    return Some(new_hash);
                }
            }else {
                if set {
                    return Some(new_hash);
                }
            }
        }
        None
    }

    fn add_entry(&mut self, key: T, _value: U) {
        let mut hash = self.get_hash(&key);

        if let Some(ref mut entry) = self.data[hash] {
            if entry.key != key { 
                hash = self.collision_handling(&key, hash, true).unwrap(); // if collsion handling return None here then the Hash Table has an error
            }
        }

        if let Some(ref mut entry) = self.data[hash] {
            entry.value = _value;
        }else {
            let pair = KeyValuePair::new(key, _value);
            self.data[hash] = Some(pair);
        }

        self.data_count += 1;
    }

    pub fn set(&mut self, key: T, value: U) {
        self.resize_or_not();
        self.add_entry(key, value);
    }

    pub fn get(&self, key: T) -> Option<&U>{
        let mut hash = self.get_hash(&key);
        if let Some(ref entry) = self.data[hash] {
            if entry.key != key {  
                hash = self.collision_handling(&key, hash, false).unwrap();
            }
        }

        if let Some(ref entry) = self.data[hash] {
            Some(&entry.value)
        }else {
            None
        }
    }

    fn get_mut(&mut self, key: T) -> Option<&mut U>{
        let mut hash = self.get_hash(&key);
        if let Some(ref entry) = self.data[hash] {
            if entry.key != key {  
                hash = self.collision_handling(&key, hash, false).unwrap();
            }
        }

        if let Some(ref mut entry) = self.data[hash] {
            Some(&mut entry.value)
        }else {
            None
        }
    }

    pub fn print(&self){
        for entry in self.data.into_iter() {
            if let Some(ref pair) = entry {
                println!("key: {}, value: {}",pair.key,pair.value);
            }else {
                println!("None")
            }
        }
    }

    pub fn remove(&mut self, key: T) {

        let mut hash = self.get_hash(&key);

        if let Some(ref entry) = self.data[hash] {
            if entry.key == key { 
                self.data[hash] = None;
                return;
            }
        }
        
        let hash_opt = self.collision_handling(&key, hash, false);

        if hash_opt.is_none() {
            return; // key is not found
        }else {
            hash = hash_opt.unwrap();
        }

        self.data[hash] = None;
    }

}

impl<T: Clone + Ord + Display, U: Clone + Ord + Display>  Index<T> for HashTable<T, U>{
    type Output = U;

    fn index(&self, index: T) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T: Clone + Ord + Display, U: Clone + Ord + Display> IndexMut<T> for HashTable<T, U> {
    fn index_mut(&mut self, index: T) -> &mut U {
        self.get_mut(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_test() {
        let mut hm: HashTable<i32,&str> = HashTable::new(6);

        hm.set(43, "hello");
        hm.set(23, "how are you");
        hm.set(12, "i am fine");
        hm.set(453, "thank you");
        hm.set(463, "how about you");

        assert_eq!(hm[43], "hello");
        assert_eq!(hm[23], "how are you");
        assert_eq!(hm[12], "i am fine");
        assert_eq!(hm[453], "thank you");
        assert_eq!(hm[463], "how about you");
    }

    #[test]
    fn resize_test() {
        let mut hm: HashTable<i32,&str> = HashTable::new(3);

        hm.set(43, "hello");
        hm.set(23, "how are you");
        hm.set(12, "i am fine");
        
        assert_eq!(hm.capacity, 3);

        hm.set(453, "thank you");

        assert_eq!(hm.capacity, 6);
    }

    #[test]
    fn hash_correctness_test() {
        let mut hm: HashTable<i32,&str> = HashTable::new(7);

        hm.set(43, "hello");
        hm.set(23, "how are you");
        hm.set(12, "i am fine");
        hm.set(453, "thank you");
        hm.set(463, "how about you");
        
        // these data i get from manual hashing of the key and detection of where they must be in the hash table
        assert_eq!(hm.data[0].is_none(), true);
        assert_eq!(hm.data[1].as_ref().unwrap().value, "hello");
        assert_eq!(hm.data[2].as_ref().unwrap().value, "i am fine");
        assert_eq!(hm.data[3].as_ref().unwrap().value, "how about you");
        assert_eq!(hm.data[4].as_ref().unwrap().value, "how are you");
        assert_eq!(hm.data[5].as_ref().unwrap().value, "thank you");
        assert_eq!(hm.data[6].is_none(), true);
    }

    #[test]
    fn remove_test() {
        let mut hm: HashTable<i32,&str> = HashTable::new(7);

        hm.set(43, "hello");
        hm.set(23, "how are you");
        hm.set(12, "i am fine");
        hm.set(453, "thank you");
        hm.set(463, "how about you");
        
        hm.remove(23);
        hm.remove(12);
        hm.remove(463);

        assert_eq!(hm.data[0].is_none(), true);
        assert_eq!(hm.data[1].as_ref().unwrap().value, "hello");
        assert_eq!(hm.data[2].is_none(), true);
        assert_eq!(hm.data[3].is_none(), true);
        assert_eq!(hm.data[4].is_none(), true);
        assert_eq!(hm.data[5].as_ref().unwrap().value, "thank you");
        assert_eq!(hm.data[6].is_none(), true);
    }
}