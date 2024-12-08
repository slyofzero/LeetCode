pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {return  0;}

    let mut jumps = 0;
    let (mut i, mut j) = (0, 0);

    while j < nums.len() - 1 {
        let mut farthest = 0 as i32;
        
        for index in i..j + 1 {
            farthest = farthest.max(index as i32 + nums[index]);
        }

        i = j + 1;
        j = farthest as usize;
        jumps += 1;
    }

    jumps
}

fn main() {
    let nums = Vec::from([10,9,8,7,6,5,4,3,2,1,1,0]);
    println!("{}", jump(nums));
}