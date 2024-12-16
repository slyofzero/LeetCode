pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![-1; nums.len()];
    let mut stack = vec![];

    for index in 0..nums.len() * 2 {
        let i = index % nums.len();
        let num = nums[i];

        while let Some(&last) = stack.last() {
            if num > nums[last] {
                ans[last] = num;
                stack.pop();
            } else {break;}
        }

        stack.push(i);
    }

    ans
}

fn main() {
    let nums = vec![1,2,3,2,1];
    println!("{:?}", next_greater_elements(nums));
}