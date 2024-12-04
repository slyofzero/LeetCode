use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    let mut max_size: i32 = 0;
    let mut chars_so_far = HashSet::<char>::new();
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;

    for end in 0..s.len() {
        while chars_so_far.contains(&chars[end]) {
            chars_so_far.remove(&chars[start]);
            start += 1;
        }

        chars_so_far.insert(chars[end]);
        max_size = max_size.max(((end - start) + 1) as i32);
    }

    max_size       
}

fn main() {
    let s = String::from("dvdf");
    let length = length_of_longest_substring(s);

    println!("{}", length);
}