// Dynamic Programming
pub fn can_jump_dp(nums: Vec<i32>) -> bool {
    let mut reachability = vec![false; nums.len()];
    reachability[0] = true;

    for i in 1..nums.len() {
        let mut j = (i - 1) as i32;

        while j >= 0 {
            let is_reachable = reachability[j as usize] && nums[j as usize] >= (i as i32) - j;
            
            if is_reachable {
                reachability[i] = is_reachable;
                break;
            }

            j -= 1;
        }
    }

    reachability[nums.len() - 1]
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut goal = (nums.len() - 1) as i32;
    let mut i = (nums.len() - 2) as i32;

    while i >= 0 {
        if nums[i as usize] >= goal - i {
            goal = i;
        }

        i -= 1;
    }

    goal == 0
}

fn main() {
    let nums = Vec::from([3,2,1,0,4]);
    println!("{}", can_jump(nums));
}