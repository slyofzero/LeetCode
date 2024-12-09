pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let size = *(&nums.len());
    let mut answer = vec![1; size];

    let mut prefix = 1;
    for i in 0..size {
        answer[i] = prefix;
        prefix *= nums[i];
    }

    let mut postfix = 1;
    for i in (0..size).rev() {
        answer[i] *= postfix;
        postfix *= nums[i];
    }

    answer
}

fn main() {
    let nums = vec![1,2,3,4];
    println!("{:?}", product_except_self(nums));
}