pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut triplets: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums.clone();
    nums.sort();

    for (i, &num) in nums.iter().enumerate() {
        if i > 0 && nums[i - 1] == num {continue;}

        let target = -1 * num;
        let (mut j, mut k) = (i + 1, nums.len() - 1);

        while j < k {
            let cur_sum = nums[j] + nums[k];

            if cur_sum < target {
                j += 1;
            } else if cur_sum > target {
                k -= 1;
            } else {
                triplets.push(vec![nums[i], nums[j], nums[k]]);
                j += 1; k -= 1;
                while nums[j] == nums[j - 1] && j < k {
                    j += 1;
                }
            }
        }
    }

    triplets
}

fn main() {
    let nums = vec![-2,0,0,2,2];
    println!("{:?}", three_sum(nums));
}
