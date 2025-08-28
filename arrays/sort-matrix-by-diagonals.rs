/*
  3446. Sort Matrix by Diagonals
  
  You are given an n x n square matrix of integers grid. Return the matrix such that:
      The diagonals in the bottom-left triangle (including the middle diagonal) are sorted in non-increasing order.
      The diagonals in the top-right triangle are sorted in non-decreasing order.
  
  Example 1:
  Input: grid = [[1,7,3],[9,8,2],[4,5,6]]
  Output: [[8,2,3],[9,6,7],[4,5,1]]
  Explanation:
  The diagonals with a black arrow (bottom-left triangle) should be sorted in non-increasing order:
      [1, 8, 6] becomes [8, 6, 1].
      [9, 5] and [4] remain unchanged.
  The diagonals with a blue arrow (top-right triangle) should be sorted in non-decreasing order:
      [7, 2] becomes [2, 7].
      [3] remains unchanged.
  
  Example 2:
  Input: grid = [[0,1],[1,2]]
  Output: [[2,1],[1,0]]
  Explanation:
  The diagonals with a black arrow must be non-increasing, so [0, 2] is changed to [2, 0]. The other diagonals are already in the correct order.
  
  Example 3:
  Input: grid = [[1]]
  Output: [[1]]
  Explanation:
  Diagonals with exactly one element are already in order, so no changes are needed.
*/
impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = 2 * n + 1;

        let mut result = vec![vec![0; n]; n];
        let mut diag = vec![Vec::with_capacity(n); m];

        for i in 0..n {
            for j in 0..n {
                diag[i - j + n].push(grid[i][j]);
            }
        }

        for i in 0..m {
            if i < n {
                diag[i].sort_by(|a, b| b.cmp(a));
            } else {
                diag[i].sort();
            }
        }

        for i in 0..n {
            for j in 0..n {
                result[i][j] = diag[i - j + n]
                    .pop()
                    .unwrap();
            }
        }

        result
    }
}
