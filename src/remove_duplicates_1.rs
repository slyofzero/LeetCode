fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {return 0;}

    let mut j = 1;

    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            nums[j] = nums[i];
            j += 1;
        }
    }

    nums.truncate(j);

    j as i32
}