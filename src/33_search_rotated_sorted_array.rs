fn search(nums: Vec<i32>, target: i32) -> i32 {
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

    let pivot = start;
    let (to_search_in, to_add) = if pivot != 0 && target >= nums[0] && target <= nums[pivot - 1] {
        (&nums[0..pivot], 0)
    } else {
        (&nums[pivot..nums.len()], pivot)
    };

    println!("{:?}", to_search_in);

    let mut start = 0;
    let mut end = to_search_in.len();

    while start < end {
        let median = (start + end) / 2;
        let median_val = to_search_in[median];

        if median_val > target {
            end = median;
        } else if median_val < target {
            start = median + 1;
        } else {
            return (median + to_add) as i32;
        }
    }
    
    -1
}

fn main() {
    let nums = vec![5,1,3];
    let target = 3;
    println!("{:?}", search(nums, target));
}