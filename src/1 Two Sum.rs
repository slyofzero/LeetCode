use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, usize>::new();

    for (index, num) in nums.into_iter().enumerate() {
        let diff = target - num;
        let diff_index = map.get(&diff);

        if diff_index.is_some() {
            return vec![index as i32, *diff_index.unwrap() as i32];
        }

        map.insert(num, index);
    }

    vec![1]
}

fn main() {}

#[test]
fn test() {
    let nums = vec![2,7,11,15];
    let target = 9;

    assert_eq!(two_sum(nums, target), vec![0, 1]);
}