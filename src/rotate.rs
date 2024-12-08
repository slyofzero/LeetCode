pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let size = nums.len();
    let k = k as usize % size;

    nums.reverse();

    nums[0..k].reverse();
    nums[k..size].reverse();
}

fn main() {
    let mut nums = Vec::from([1,2,3,4,5]);
    let k = 3;
    rotate(&mut nums, k);

    println!("{:?}", nums);
}