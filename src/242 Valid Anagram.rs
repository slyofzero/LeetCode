pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    
    let mut char_counts = [0; 26];

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        char_counts[(s_char as u8 - b'a') as usize] += 1;
        char_counts[(t_char as u8 - b'a') as usize] -= 1;
    }

    char_counts.iter().all(|&x| x == 0)
}

fn main() {}

#[test]
fn test() {
    let s = String::from("rat");
    let t = String::from("car");
    assert_eq!(is_anagram(s, t), false);
}