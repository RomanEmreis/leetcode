/*
  2435. Paths in Matrix Whose Sum Is Divisible by K
  
  You are given a 0-indexed m x n integer matrix grid and an integer k. 
  You are currently at position (0, 0) and you want to reach position (m - 1, n - 1) moving only down or right.
  
  Return the number of paths where the sum of the elements on the path is divisible by k. 
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
  Output: 2
  Explanation: There are two paths where the sum of the elements on the path is divisible by k.
  The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
  The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
  
  Example 2:
  Input: grid = [[0,0]], k = 5
  Output: 1
  Explanation: The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
  
  Example 3:
  Input: grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
  Output: 10
  Explanation: Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
*/
const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        let mut dp = vec![vec![[0; 50]; n]; m];
        dp[0][0][grid[0][0] as usize % k] = 1;

        for i in 0..m {
            for j in 0..n {
                for sum in 0..k {
                    let new_sum = (sum + grid[i][j] as usize) % k;
                    if i > 0 {
                        dp[i][j][new_sum] += dp[i - 1][j][sum];
                    }
                    if j > 0 {
                        dp[i][j][new_sum] += dp[i][j - 1][sum];
                    }
                    dp[i][j][new_sum] %= MOD;
                }
            }
        }

        dp[m - 1][n - 1][0]
    }
}
