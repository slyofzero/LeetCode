use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut greater_than_map = HashMap::new();
    let mut stack = vec![];

    for i in (0..nums2.len()).rev() {
        while let Some(&top_elem) = stack.last() {
            if top_elem <= nums2[i] {
                stack.pop();
            } else {
                greater_than_map.insert(nums2[i], top_elem);
                break;
            }
        }

        stack.push(nums2[i]);
    }

    let mut ans = vec![];

    for num in nums1 {
        ans.push(*greater_than_map.get(&num).unwrap_or(&-1));
    }

    ans
}

fn main() {
    let nums1 = vec![4,1,2];
    let nums2 = vec![1,3,4,2];
    println!("{:?}", next_greater_element(nums1, nums2));
}