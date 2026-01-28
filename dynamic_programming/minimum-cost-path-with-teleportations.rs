/*
  3651. Minimum Cost Path with Teleportations
  
  You are given a m x n 2D integer array grid and an integer k. You start at the top-left cell (0, 0) and your goal is to reach the bottom‐right cell (m - 1, n - 1).
  There are two types of moves available:
      Normal move: You can move right or down from your current cell (i, j), i.e. you can move to (i, j + 1) (right) or (i + 1, j) (down). The cost is the value of the destination cell.
      Teleportation: You can teleport from any cell (i, j), to any cell (x, y) such that grid[x][y] <= grid[i][j]; the cost of this move is 0. You may teleport at most k times.
  
  Return the minimum total cost to reach cell (m - 1, n - 1) from (0, 0).
  
  Example 1:
  Input: grid = [[1,3,3],[2,5,4],[4,3,5]], k = 2
  Output: 7
  Explanation:
  Initially we are at (0, 0) and cost is 0.
  Current Position	 Move	       New Position	Total Cost
  (0, 0)	         Move Down	      (1, 0)	   0 + 2 = 2
  (1, 0)	         Move Right	      (1, 1)	   2 + 5 = 7
  (1, 1)	      Teleport to (2, 2)	(2, 2)	   7 + 0 = 7
  
  The minimum cost to reach bottom-right cell is 7.
  
  Example 2:
  Input: grid = [[1,2],[2,3],[3,4]], k = 1
  Output: 9
  Explanation:
  Initially we are at (0, 0) and cost is 0.
  Current Position	Move	New Position	Total Cost
  (0, 0)	      Move Down	  (1, 0)	    0 + 2 = 2
  (1, 0)	      Move Right	(1, 1)	    2 + 3 = 5
  (1, 1)	      Move Down	  (2, 1)    	5 + 4 = 9
  
  The minimum cost to reach bottom-right cell is 9.
*/
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_v = 0;

        for i in 0..m {
            for j in 0..n {
                max_v = max_v.max(grid[i][j]);
            }
        }

        let mut dp = vec![vec![0; n]; m];
        
        let mut temp = vec![i32::MAX; (max_v + 1) as usize];
        let mut best = vec![0; (max_v + 1) as usize];

        temp[grid[m - 1][n - 1] as usize] = 0;

        for i in (0..n - 1).rev() {
            dp[m - 1][i] = dp[m - 1][i + 1] + grid[m - 1][i + 1];
            let idx = grid[m - 1][i] as usize;
            temp[idx] = temp[idx].min(dp[m - 1][i]);
        }

        for i in (0..m - 1).rev() {
            dp[i][n - 1] = dp[i + 1][n - 1] + grid[i + 1][n - 1];
            let idx = grid[i][n - 1] as usize;
            temp[idx] = temp[idx].min(dp[i][n - 1]);

            for j in (0..n - 1).rev() {
                dp[i][j] = (dp[i + 1][j] + grid[i + 1][j])
                    .min(dp[i][j + 1] + grid[i][j + 1]);
                let idx = grid[i][j] as usize;
                temp[idx] = temp[idx].min(dp[i][j]);
            }
        }

        for _ in 0..k {
            best[0] = temp[0];
            for i in 1..=max_v as usize {
                best[i] = best[i - 1].min(temp[i]);
            }

            for i in (0..n - 1).rev() {
                dp[m - 1][i] = best[grid[m - 1][i] as usize]
                    .min(dp[m - 1][i + 1] + grid[m - 1][i + 1]);
                let idx = grid[m - 1][i] as usize;
                temp[idx] = temp[idx].min(dp[m - 1][i]);
            }

            for i in (0..m - 1).rev() {
                dp[i][n - 1] = best[grid[i][n - 1] as usize]
                    .min(dp[i + 1][n - 1] + grid[i + 1][n - 1]);
                let idx = grid[i][n - 1] as usize;
                temp[idx] = temp[idx].min(dp[i][n - 1]);

                for j in (0..n - 1).rev() {
                    dp[i][j] = (dp[i + 1][j] + grid[i + 1][j])
                        .min(dp[i][j + 1] + grid[i][j + 1])
                        .min(best[grid[i][j] as usize]);
                    let idx = grid[i][j] as usize;
                    temp[idx] = temp[idx].min(dp[i][j]);
                }
            }
        }

        dp[0][0]
    }
}
