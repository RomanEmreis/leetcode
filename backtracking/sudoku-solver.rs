/*
  37. Sudoku Solver
  
  Write a program to solve a Sudoku puzzle by filling the empty cells.
  A sudoku solution must satisfy all of the following rules:
      Each of the digits 1-9 must occur exactly once in each row.
      Each of the digits 1-9 must occur exactly once in each column.
      Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
  
  The '.' character indicates empty cells.
  
  Example 1:
  Input: board = [["5","3",".",".","7",".",".",".","."],
                  ["6",".",".","1","9","5",".",".","."],
                  [".","9","8",".",".",".",".","6","."],
                  ["8",".",".",".","6",".",".",".","3"],
                  ["4",".",".","8",".","3",".",".","1"],
                  ["7",".",".",".","2",".",".",".","6"],
                  [".","6",".",".",".",".","2","8","."],
                  [".",".",".","4","1","9",".",".","5"],
                  [".",".",".",".","8",".",".","7","9"]]
  Output: [["5","3","4","6","7","8","9","1","2"],
           ["6","7","2","1","9","5","3","4","8"],
           ["1","9","8","3","4","2","5","6","7"],
           ["8","5","9","7","6","1","4","2","3"],
           ["4","2","6","8","5","3","7","9","1"],
           ["7","1","3","9","2","4","8","5","6"],
           ["9","6","1","5","3","7","2","8","4"],
           ["2","8","7","4","1","9","6","3","5"],
           ["3","4","5","2","8","6","1","7","9"]]
  Explanation: The input board is shown above and the only valid solution is shown below:
*/
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; N];
        let mut cols = [0u16; N];
        let mut boxes_m = [0u16; N];
        let mut empties_store = [(0_usize, 0_usize); N * N];
        let mut num_empties = 0;

        for r in 0..N {
            for c in 0..N {
                let ch = board[r][c];
                if ch == '.' {
                    empties_store[num_empties] = (r, c);
                    num_empties += 1
                } else {
                    let d = (ch as u8 - b'0') as usize;
                    let bit = 1u16 << d;
                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes_m[box_index(r, c)] |= bit;
                }
            }
        }

        let empties = &mut empties_store[..num_empties];
        let _ = dfs(board, &mut rows, &mut cols, &mut boxes_m, empties, 0);
    }
}

const N: usize = 9;
const ALL_MASK: u16 = 0b1111111110;

fn box_index(r: usize, c: usize) -> usize {
    (r / 3) * 3 + (c / 3)
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    rows: &mut [u16; N],
    cols: &mut [u16; N],
    boxes_m: &mut [u16; N],
    empties: &mut [(usize, usize)],
    idx: usize,
) -> bool {
    if idx == empties.len() {
        return true;
    }
    let mut best = idx;
    let mut best_count = 10u32;
    for (k, &empty) in empties.iter().enumerate().skip(idx) {
        let (r, c) = empty;
        let b = box_index(r, c);
        let mask = !(rows[r] | cols[c] | boxes_m[b]) & ALL_MASK;
        let cnt = mask.count_ones();
        if cnt < best_count {
            best = k;
            best_count = cnt;
            if cnt == 1 {
                break;
            }
        }
    }
    if best_count == 0 {
        return false;
    }

    empties.swap(idx, best);

    let (r, c) = empties[idx];
    let b = box_index(r, c);
    let mask = !(rows[r] | cols[c] | boxes_m[b]) & ALL_MASK;

    for d in 1..=N {
        let bit = 1u16 << d;
        if mask & bit == 0 {
            continue;
        }

        board[r][c] = char::from(b'0' + d as u8);
        rows[r] |= bit;
        cols[c] |= bit;
        boxes_m[b] |= bit;

        if dfs(board, rows, cols, boxes_m, empties, idx + 1) {
            return true;
        }

        board[r][c] = '.';
        rows[r] &= !bit;
        cols[c] &= !bit;
        boxes_m[b] &= !bit;
    }

    false
}
