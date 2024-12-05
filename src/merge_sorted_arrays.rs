#![allow(dead_code, unused_variables)]

fn partition(nums: &mut Vec<i32>, l: usize, h: usize) -> usize {
    let pivot = nums[l];
    let (mut i, mut j) = (l + 1, h);

    while i <= j {
        loop {
            if i > h || nums[i] >= pivot  { break; }
            i += 1;
        }

        loop {
            if j <= l || nums[j] < pivot { break; }
            j -= 1;
        }

        if i < j { nums.swap(i, j); }
    }

    nums.swap(l, j);

    j
}

fn quick_sort(nums: &mut Vec<i32>, l: usize, h: usize) {
    if h > l {
        let j = partition(nums, l, h);
        quick_sort(nums, l, j);
        quick_sort(nums, j + 1, h);
    }
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (m, n) = (m as usize, n as usize);
    let total_size = m+n;

    for i in m..total_size {
        nums1[i] = nums2[i - m];
    }

    quick_sort(nums1, 0, total_size - 1);
}

fn main() {
    let mut nums1 = Vec::from([1,2,3,0,0,0]);
    let mut nums2 = Vec::from([2,5,6]);
    let m = 3;
    let n = 3;

    merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1);
}

fn test_sort(nums: &mut Vec<i32>) -> bool {
    let h = nums.len() - 1;
    quick_sort(nums, 0, h);

    for i in 1..nums.len() - 1 {
        if nums[i - 1] > nums[i] { return false }
    }

    true
}

#[test]
fn test() {
    let mut nums1 = Vec::from([1,2,3,2,5,6]);
    let mut nums2 = Vec::from([10,16,8,12,15,6,3,9,5]);
    let mut nums3 = Vec::from([10,16,8,12,15,6,3,9,5]);
    let mut nums4 = Vec::from([0,0,3,-1,1,1,1,2,3]);

    assert_eq!(true, test_sort(&mut nums1));
    assert_eq!(true, test_sort(&mut nums2));
    assert_eq!(true, test_sort(&mut nums3));
    assert_eq!(true, test_sort(&mut nums4));
}