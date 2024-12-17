pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut s = 0;
    let mut e = nums.len();

    while s < e {
        let median = (s + e) / 2;
        let val = nums[median];

        if val > target {
            e = median
        } else if val < target {
            s = median + 1;
        } else {
            return median as i32;
        }
    }

    -1
}

fn main() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 2;
    println!("{}", search(nums, target))
}