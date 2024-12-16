pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut output = vec![0; temperatures.len()];
    let mut stack: Vec<(usize, i32)> = vec![];

    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[i] > stack.last().unwrap().1 {
            let l_index = stack.pop().unwrap().0;
            output[l_index] = (i - l_index) as i32;
        }
        stack.push((i, temperatures[i]));
    }

    output
}

fn main() {
    let temperature = vec![73,74,75,71,69,72,76,73];
    println!("{:?}", daily_temperatures(temperature));
}