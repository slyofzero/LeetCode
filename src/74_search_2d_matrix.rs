pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut to_search_in = 0;

    for i in 1..matrix.len() {
        if target >= matrix[i][0] {
            to_search_in = i;
        } else {
            break;
        }
    }

    let to_search_row = &matrix[to_search_in];
    let mut start = 0 as usize;
    let mut end = to_search_row.len();

    while start < end {
        let median = (start + end) / 2;
        let median_value = to_search_row[median];

        if target > median_value {
            start = median + 1;
        } else if target < median_value {
            end = median;
        } else {
            return true;
        }
    }

    false
}

fn main() {
    let matrix = Vec::from([[1],[3]].map(|x| Vec::from(x)));
    let target = 3;
    println!("{}", search_matrix(matrix, target));
}