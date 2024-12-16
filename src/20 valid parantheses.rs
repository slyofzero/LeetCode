pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new();

    for bracket in s.chars() {
        if bracket == '(' || bracket == '[' || bracket == '{' {
            stack.push(bracket);
        } else if bracket == ')' || bracket == ']' || bracket == '}' {
            if let Some(opening_bracket) = stack.pop() {
                // println!("{} {}", bracket, opening_bracket);
                if !((bracket == ')' && opening_bracket == '(') || bracket == ']' && opening_bracket == '[' || bracket == '}' && opening_bracket == '{') {
                    return false;
                }
            } else {
                return false;
            }
        } 
    }

    stack.is_empty()
}

fn main() {
    let s = String::from("(]");
    println!("{}", is_valid(s));
}