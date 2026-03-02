/*
  1536. Minimum Swaps to Arrange a Binary Grid
  
  Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.
  A grid is said to be valid if all the cells above the main diagonal are zeros.
  
  Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.
  
  The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).
  
  Example 1:
  Input: grid = [[0,0,1],[1,1,0],[1,0,0]]
  Output: 3
  
  Example 2:
  Input: grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
  Output: -1
  Explanation: All rows are similar, swaps have no effect on the grid.
  
  Example 3:
  Input: grid = [[1,0,0],[1,1,0],[1,1,1]]
  Output: 0
*/
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut suffix_zeros: Vec<usize> = grid
            .into_iter()
            .map(|row| get_suffix_zeros(row, n))
            .collect();
        
        let mut res = 0i32;

        for i in 0..n {
            let needed_zeros = n - i - 1;
            let Some(j) = (i..n).find(|&j| suffix_zeros[j] >= needed_zeros) else {
                return -1;
            };

            suffix_zeros[i..=j].rotate_right(1);
            res += (j - i) as i32;
        }

        res
    }
}

#[inline(always)]
fn get_suffix_zeros(row: Vec<i32>, n: usize) -> usize {
    row.into_iter()
        .rposition(|x| x == 1)
        .map_or(n, |pos| n - pos - 1)
}
