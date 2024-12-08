pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut h_index: usize = 0;
    
    for h in 1..&citations.len() + 1 {
        let mut count = 0;
        for cite in &citations {
            if *cite >= h as i32 {
                count += 1;
            }

            if count >= h {
                h_index = h;
                break;
            }
        }

        if count < h {
            break;
        }
    }

    h_index as i32
}

fn main() {
    let citations = vec![1,3,1];
    println!("{}", h_index(citations));
}