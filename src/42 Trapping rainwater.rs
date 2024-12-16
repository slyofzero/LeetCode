pub fn trap(height: Vec<i32>) -> i32 {
    let mut max_left = vec![0; height.len()];
    let mut max_right = vec![0; height.len()];
    let mut trapped_water = 0;

    for i in 1..height.len() {
        max_left[i] = height[i - 1].max(max_left[i - 1]);
    }

    for i in (0..height.len() - 1).rev() {
        max_right[i] = height[i + 1].max(max_right[i + 1]);
    }

    for i in 0..height.len() {
        let water = max_left[i].min(max_right[i]) - height[i];
        if water > 0 {
            trapped_water += water;
        }
    }

    trapped_water
}

fn main() {
    let height = vec![0];
    println!("{:?}", trap(height));
}