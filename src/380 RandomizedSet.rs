use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    vals: Vec<i32>,
    map: HashMap<i32, usize>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet { vals: Vec::new(), map: HashMap::<i32, usize>::new() }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let val_exists = self.map.get(&val);
        if val_exists.is_none() {
            self.map.insert(val, self.vals.len());
            self.vals.push(val);
            return true;
        }

        false
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&val_index) = self.map.get(&val) {
            self.map.remove(&val);
            let last_item = self.vals[self.vals.len() - 1];
            self.vals.swap_remove(val_index);
            
            if val_index < self.vals.len() {
                self.map.insert(last_item, val_index);
            }

            return true;
        }
        
        false
    }
    
    fn get_random(&self) -> i32 {
        let index = rand::thread_rng().gen_range(0..self.vals.len());
        self.vals[index]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

fn main() {

}

#[test]
fn test() {
    let mut rand_set = RandomizedSet::new();

    assert_eq!(rand_set.insert(0), true);
    assert_eq!(rand_set.insert(1), true);
    println!("{:?} {:?}", rand_set.vals, rand_set.map);

    assert_eq!(rand_set.remove(0), true);
    println!("{:?} {:?}", rand_set.vals, rand_set.map);
    assert_eq!(rand_set.insert(2), true);
    assert_eq!(rand_set.remove(1), true);
    println!("{:?} {:?}", rand_set.vals, rand_set.map);


    assert_eq!(rand_set.get_random(), 2);
}