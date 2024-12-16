pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut data: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    data.sort_by(|x, y| x.0.cmp(&y.0));
    let mut stack: Vec<i32> = vec![];

    for i in (0..data.len()).rev() {
        let (p, s) = data[i];
        stack.push((target - p) / s);

        if stack.len() >= 2 && stack[stack.len() - 1] <= stack[stack.len() - 2] {
            stack.pop();
        }
    }

    stack.len() as i32
}

fn main() {
    let position = vec![10,8,0,5,3];
    let speed = vec![2,4,1,1,3];
    let target = 12;

    println!("{}", car_fleet(target, position, speed));
}