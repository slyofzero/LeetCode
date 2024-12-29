use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap { data: HashMap::new() }
    }
    
    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_insert_with(Vec::new).push((timestamp, value));
    }
    
    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(timestamps) = self.data.get(&key) {
            let mut biggest = timestamps.len();
            let mut start = 0;
            let mut end = timestamps.len();

            while start < end {
                let median = (start + end) / 2;
                let median_val = timestamps[median].0;

                if median_val <= timestamp {
                    biggest = median;
                    start = median + 1;
                } else {
                    end = median;
                }
            }

            if let Some(output) = timestamps.get(biggest).and_then(|s| Some(s.1.to_string())) {
                output
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {
    let methods = ["set","set","get","get","get","get","get"];
    let parameters = [vec!["love","high","10"],vec!["love","low","20"],vec!["love","5"],vec!["love","10"],vec!["love","15"],vec!["love","20"],vec!["love","25"]];

    let mut timemap = TimeMap::new();
    let mut output_string = String::new();
    output_string += "null,";

    for (i, method) in methods.iter().enumerate() {
        let param = &parameters[i];

        if *method == "set" {
            let timestamp: i32 = param[2].parse().unwrap();
            timemap.set(param[0].to_string(), param[1].to_string(), timestamp);
            output_string += "null,"
        } else if *method == "get" {
            let timestamp: i32 = param[1].parse().unwrap();
            let output = timemap.get(param[0].to_string(), timestamp);
            output_string.push_str(&format!("\"{}\",", output));
        }
    }

    println!("{}", output_string);
}