/*
  1594. Maximum Non Negative Product in a Matrix
  
  You are given a m x n matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.
  Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (m - 1, n - 1), 
  find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.
  
  Return the maximum non-negative product modulo 109 + 7. If the maximum product is negative, return -1.
  
  Notice that the modulo is performed after getting the maximum product.
  
  Example 1:
  Input: grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
  Output: -1
  Explanation: It is not possible to get non-negative product in the path from (0, 0) to (2, 2), so return -1.
  
  Example 2:
  Input: grid = [[1,-2,1],[1,-2,1],[3,-4,1]]
  Output: 8
  Explanation: Maximum non-negative product is shown (1 * 1 * -2 * -4 * 1 = 8).
  
  Example 3:
  Input: grid = [[1,3],[0,-4]]
  Output: 0
  Explanation: Maximum non-negative product is shown (1 * 0 * -4 = 0).
*/
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp_min = vec![vec![0i64; n]; m];
        let mut dp_max = vec![vec![0i64; n]; m];

        dp_min[0][0] = grid[0][0] as i64;
        dp_max[0][0] = grid[0][0] as i64;

        for i in 1..m {
            let v = grid[i][0] as i64;
            dp_min[i][0] = dp_min[i - 1][0] * v;
            dp_max[i][0] = dp_max[i - 1][0] * v;
        }

        for j in 1..n {
            let v = grid[0][j] as i64;
            dp_min[0][j] = dp_min[0][j - 1] * v;
            dp_max[0][j] = dp_max[0][j - 1] * v;
        }

        for i in 1..m {
            for j in 1..n {
                let v = grid[i][j] as i64;
                let candidates = [
                    dp_min[i - 1][j] * v,
                    dp_max[i - 1][j] * v,
                    dp_min[i][j - 1] * v,
                    dp_max[i][j - 1] * v,
                ];
                dp_min[i][j] = *candidates.iter().min().unwrap();
                dp_max[i][j] = *candidates.iter().max().unwrap();
            }
        }

        let result = dp_max[m - 1][n - 1];
        if result >= 0 {
            (result % MOD) as i32
        } else {
            -1
        }
    }
}
