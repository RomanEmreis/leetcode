/*
  1895. Largest Magic Square
  
  A k x k magic square is a k x k grid filled with integers such that every row sum, every column sum, and both diagonal sums are all equal. 
  The integers in the magic square do not have to be distinct. Every 1 x 1 grid is trivially a magic square.
  Given an m x n integer grid, return the size (i.e., the side length k) of the largest magic square that can be found within this grid.
  
  Example 1:
  Input: grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]
  Output: 3
  Explanation: The largest magic square has a size of 3.
  Every row sum, column sum, and diagonal sum of this magic square is equal to 12.
  - Row sums: 5+1+6 = 5+4+3 = 2+7+3 = 12
  - Column sums: 5+5+2 = 1+4+7 = 6+3+3 = 12
  - Diagonal sums: 5+4+3 = 6+4+2 = 12
  
  Example 2:
  Input: grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]]
  Output: 2
*/
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut p_row = vec![vec![0; n + 1]; m];
        let mut p_col = vec![vec![0; m + 1]; n];

        for i in 0..m {
            for j in 0..n {
                p_row[i][j + 1] = p_row[i][j] + grid[i][j];
                p_col[j][i + 1] = p_col[j][i] + grid[i][j];
            }
        }

        let mut k = m.min(n);
        while k >= 2 {
            if contains(&grid, &p_row, &p_col, k) {
                return k as i32;
            }
            k -= 1;
        } 

        1
    }
}

fn contains(grid: &[Vec<i32>], p_row: &[Vec<i32>], p_col: &[Vec<i32>], k: usize) -> bool {
    for i in 0..=(grid.len() - k) {
        for j in 0..=(grid[0].len() - k) {
            if is_magic(grid, p_row, p_col, i, j, k) {
                return true;
            }
        }
    }

    false
}

fn is_magic(grid: &[Vec<i32>], p_row: &[Vec<i32>], p_col: &[Vec<i32>], i: usize, j: usize, k: usize) -> bool {
    let mut diag = 0;
    let mut anti = 0;

    for d in 0..k {
        diag += grid[i + d][j + d];
        anti += grid[i + d][j + k - 1 - d];
    }

    if diag != anti {
        return false;
    }

    for d in 0..k {
        if get_sum(p_row, i + d, j, j + k - 1) != diag {
            return false;
        }
        if get_sum(p_col, j + d, i, i + k - 1) != diag {
            return false;
        }
    }

    true
}

fn get_sum(p: &[Vec<i32>], i: usize, l: usize, r: usize) -> i32 {
    p[i][r + 1] - p[i][l]
}
