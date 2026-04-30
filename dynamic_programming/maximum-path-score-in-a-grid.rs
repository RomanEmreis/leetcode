/*
  3742. Maximum Path Score in a Grid
  
  You are given an m x n grid where each cell contains one of the values 0, 1, or 2. You are also given an integer k.
  You start from the top-left corner (0, 0) and want to reach the bottom-right corner (m - 1, n - 1) by moving only right or down.
  
  Each cell contributes a specific score and incurs an associated cost, according to their cell values:
      0: adds 0 to your score and costs 0.
      1: adds 1 to your score and costs 1.
      2: adds 2 to your score and costs 1. ​​​​​​​
  
  Return the maximum score achievable without exceeding a total cost of k, or -1 if no valid path exists.
  
  Note: If you reach the last cell but the total cost exceeds k, the path is invalid.
  
  Example 1:
  Input: grid = [[0, 1],[2, 0]], k = 1
  Output: 2
  Explanation:​​​​​​​
  The optimal path is:
  Cell	grid[i][j]	Score	Total
  Score	Cost	Total
  Cost
  (0, 0)	0	0	0	0	0
  (1, 0)	2	2	2	1	1
  (1, 1)	0	0	2	0	1
  
  Thus, the maximum possible score is 2.
  
  Example 2:
  Input: grid = [[0, 1],[1, 2]], k = 1
  Output: -1
  Explanation:
  There is no path that reaches cell (1, 1)​​​​​​​ without exceeding cost k. Thus, the answer is -1.
*/
impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        let mut prev = vec![vec![-1_i32; k + 1]; n];
        let mut curr = vec![vec![-1_i32; k + 1]; n];

        prev[0][0] = 0;

        for j in 1..n {
            let val = grid[0][j];
            let cost = usize::from(val != 0);
            for c in cost..=k {
                let from_left = prev[j - 1][c - cost];
                if from_left >= 0 {
                    prev[j][c] = from_left + val;
                }
            }
        }

        for i in 1..m {
            for row in curr.iter_mut() {
                row.fill(-1);
            }

            for j in 0..n {
                let val = grid[i][j];
                let cost = usize::from(val != 0);

                for c in cost..=k {
                    let pc = c - cost;
                    let mut best = prev[j][pc];
                    if j > 0 && curr[j - 1][pc] > best {
                        best = curr[j - 1][pc];
                    }
                    if best >= 0 {
                        curr[j][c] = best + val;
                    }
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        prev[n - 1].iter().copied().max().unwrap_or(-1).max(-1)
    }
}
