pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::<String>::new();

    for token in tokens {
        if token.parse::<i32>().is_ok() {
            stack.push(token);
        } else {
            if let (Some(b_str), Some(a_str)) = (stack.pop(), stack.pop()) {
                let (a, b) = (a_str.parse::<i32>().unwrap(), b_str.parse::<i32>().unwrap());
                let new_num = match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => 0
                };
                stack.push(new_num.to_string());
            }
        }
    }

    stack[0].parse::<i32>().unwrap()
}

fn main() {
    let tokens = Vec::from(["10","6","9","3","+","-11","*","/","*","17","+","5","+"].map(|s| String::from(s)));
    println!("{:?}", eval_rpn(tokens));
}