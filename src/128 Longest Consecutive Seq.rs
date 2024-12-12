use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 0;

    for num in &set {
        if *&set.get(&(num - 1)).is_none() {
            let mut length = 0;

            while *&set.get(&(num + length)).is_some() {
                length += 1;
            }


            longest = longest.max(length)
        }
    }

    longest
}

fn main() {
    let nums = vec![100,4,200,1,3,2];
    println!("{:?}", longest_consecutive(nums));
}