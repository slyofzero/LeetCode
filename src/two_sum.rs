use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    let nums = Vec::from([3,2,4]);
    let target = 6;

    println!("{:?}", two_sum(nums, target));

    println!("Hello, world!");
}
