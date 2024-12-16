pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let (mut l, mut r) = (0, height.len() - 1);

    while l < r {
        let area = height[l].min(height[r]) * (r - l) as i32;
        max_area = max_area.max(area);

        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max_area
}

fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    println!("{}", max_area(height));
}