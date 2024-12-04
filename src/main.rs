fn partition(nums: &mut Vec<i32>, l: usize, h: usize) -> usize {
    let pivot = nums[l];
    let (mut i, mut j) = (l, h);

    // println!("{:?}", &nums[i..j+1]);

    while i < j {
        while nums[i] <= pivot && i < h {
            i += 1;
        }

        while nums[j] > pivot {
            j -= 1;
        }
        
        if i < j {
            nums.swap(i, j); 
        }
    }

    nums.swap(l, j);

    j
}

fn quick_sort(nums: &mut Vec<i32>, l: usize, h: usize) {
    // println!("{} {}", l, h);

    if l < h && nums[l] != nums[h] {
        let j = partition(nums, l, h);
        quick_sort(nums, l, j);
        quick_sort(nums, j + 1, h);
    }
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let new_size = (m + n) as usize;

    for i in (m as usize)..new_size {
        nums1[i as usize] = nums2[i - (m as usize)];
    }

    let (l, h) = (0, nums1.len() - 1);
    quick_sort(nums1, l, h);
}

fn main() {
    let mut nums1 = Vec::from([0]);
    let mut nums2 = Vec::from([1]);
    let m = 0;
    let n = 1;

    merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1);
}

// fn main() {
//     let mut nums = Vec::from([1, 2, 3, 2, 5, 6]);
//     let (l, h) = (0, nums.len() - 1);
//     quick_sort(&mut nums, l, h);

//     println!("{:?}", nums);
// }