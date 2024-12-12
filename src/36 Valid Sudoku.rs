use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::<char>::new(); 9];
    let mut cols = vec![HashSet::<char>::new(); 9];
    let mut subgrids = vec![HashSet::<char>::new(); 9];
    
    for r in 0..9 {
        for c in 0..9 {
            let item = board[r][c];
            if item == '.' {continue;}

            let subgrid_index = (r / 3) * 3 + (c / 3);

            if rows[r].contains(&item) || cols[c].contains(&item) || subgrids[subgrid_index].contains(&item) {
                return false;
            }

            rows[r].insert(item);
            cols[c].insert(item);
            subgrids[subgrid_index].insert(item);
        }
    }

    true
}

fn main() {}

#[test]
fn test() {
    let board = [["5","3",".",".","7",".",".",".","."]
                                ,["6",".",".","1","9","5",".",".","."]
                                ,[".","9","8",".",".",".",".","6","."]
                                ,["8",".",".",".","6",".",".",".","3"]
                                ,["4",".",".","8",".","3",".",".","1"]
                                ,["7",".",".",".","2",".",".",".","6"]
                                ,[".","6",".",".",".",".","2","8","."]
                                ,[".",".",".","4","1","9",".",".","5"]
                                ,[".",".",".",".","8",".",".","7","9"]];

    let mut new_board = Vec::<Vec<char>>::new();
    for row in board {
        let new_row = Vec::from(row.map(|x| x.chars().next().unwrap()));
        new_board.push(new_row);
    }

    assert_eq!(is_valid_sudoku(new_board), true);
}