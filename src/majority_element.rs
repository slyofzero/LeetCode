// Works on the assumption that a majority element DOES EXIST in the array (count is n/2)
// Boyer-Moore voting algorithm that +1 each time the candidate is present and -1 each time the current number isn't the candidate
// Basically what happens is that the majority element would always have count >= 1 because of this as it's available n/2 times
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut candidate = nums[0];
    let mut count = 0;

    for i in &nums {
        if count == 0 {
            candidate = *i;
        }

        count += if *i == candidate { 1 } else { -1 }
    }

    candidate
}

fn main() {
    let nums = Vec::from([2,2,2,1,1]);
    let major = majority_element(nums);
    println!("{:?}", major);
}