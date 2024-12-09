use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique_vals = HashSet::<i32>::new();

    for num in &nums {
        unique_vals.insert(*num);
    }

    unique_vals.len() != nums.len()
}

fn main() {

}

#[test]
fn test() {
    let nums = vec![1,2,3,1];
    assert_eq!(contains_duplicate(nums), true);
}