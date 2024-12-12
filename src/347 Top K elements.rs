use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::<i32, usize>::new();

    for num in &nums {
        *counts.entry(*num).or_insert(0) += 1;
    }

    let mut count_arr: Vec<Vec<i32>> = (0..nums.len()).map(|_| vec![]).collect();

    for (num, count) in counts {
        count_arr[count - 1].push(num);
    }

    let answer = count_arr.iter().rev().filter(|&x| !x.is_empty()).flatten().copied().collect::<Vec<_>>();
    answer[0..k as usize].to_vec()
}

fn main() {
    let nums = vec![1,1,1,2,2,3];
    let k = 2;

    println!("{:?}", top_k_frequent(nums, k));
}