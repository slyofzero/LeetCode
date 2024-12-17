pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {return 0;}

    let (mut l, mut r) = (n, 0);
    let (mut max_so_far, mut min_so_far) = (i32::MIN, i32::MAX);

    for i in 0..n {
        let j = n - i - 1;
        if nums[i] < max_so_far {
            r = i;
        }
        
        if nums[j] > min_so_far {
            l = j;
        }
        max_so_far = max_so_far.max(nums[i]);
        min_so_far = min_so_far.min(nums[j]);
    }

    if l >= r {return 0;}
    (r - l + 1) as i32
}

fn main() {
    let nums = vec![1,3,3,2,4];
    println!("{}", find_unsorted_subarray(nums));
}