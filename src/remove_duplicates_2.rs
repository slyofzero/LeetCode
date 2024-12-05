pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {return 0;}

    let mut j = 1;
    let mut count = 1;

    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            count = 1;
        } else {
            count += 1;
        }
        
        if count <= 2 {
            nums[j] = nums[i];
            j += 1;
        }
    }

    nums.truncate(j);

    j as i32
}

fn main() {
    let mut nums = Vec::from([1,1,1,2,2,3]);
    let j = remove_duplicates(&mut nums) as usize;
    // println!("{:?}", nums);
    println!("{:?}", &nums[0..j]);
}