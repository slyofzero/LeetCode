use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<[i32; 26], Vec<String>>::new();

    for str in strs {
        let mut char_count = [0; 26];
        for char in str.chars() {
            char_count[(char as u8 - b'a') as usize] += 1;
        }

        map.entry(char_count).or_insert_with(Vec::new).push(str);
    }

    map.into_values().collect()
}

fn main() {
    let strs = Vec::from(["eat","tea","tan","ate","nat","bat"].map(|x| String::from(x)));
    println!("{:?}", group_anagrams(strs));
}