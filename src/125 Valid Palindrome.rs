pub fn is_palindrome(s: String) -> bool {
    let (mut l, mut r) = (0, s.len() - 1);

    while l < r {
        while l < r && !s.as_bytes()[l].is_ascii_alphanumeric() {
            l += 1;
        }

        println!("{}", l);

        while l < r && !s.as_bytes()[r].is_ascii_alphanumeric() {
            r -= 1;
        }

        println!("{}", r);

        if s.as_bytes()[l].to_ascii_lowercase() != s.as_bytes()[r].to_ascii_lowercase() {
            return false;
        }

        if r == 0 { break; }
        l += 1; r -= 1;
    }

    true
}

fn main() {
}

#[test]
fn test() {
    let str = String::from("a.");
    assert_eq!(is_palindrome(str), true);
}