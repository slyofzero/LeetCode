pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut start  = 0;
    let mut end = nums.len() - 1;

    while nums[start] > nums[end] {
        let median = (start + end) / 2;
        let median_val = nums[median];

        if nums[start] > median_val {
            end = median;
        } else {
            start = median + 1;
        }
    }

    nums[start]
}

fn main() {
    let nums = vec![6,7,1,2,3];
    // let nums = vec![5,6,7,1,2,3,4];
    // let nums = vec![3,4,6,7,1,2];
    println!("{:?}", find_min(nums));
}