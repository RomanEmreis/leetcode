/*
  3546. Equal Sum Grid Partition I
  
  You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible to make either one horizontal or one vertical cut on the grid such that:
      Each of the two resulting sections formed by the cut is non-empty.
      The sum of the elements in both sections is equal.
  
  Return true if such a partition exists; otherwise return false.
  
  Example 1:
  Input: grid = [[1,4],[2,3]]
  Output: true
  Explanation:
  A horizontal cut between row 0 and row 1 results in two non-empty sections, each with a sum of 5. Thus, the answer is true.
  
  Example 2:
  Input: grid = [[1,3],[2,4]]
  Output: false
  Explanation:
  No horizontal or vertical cut results in two non-empty sections with equal sums. Thus, the answer is false.
*/
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        
        let mut p = 0_i64;
        let mut h = vec![0_i64; m];
        let mut v = vec![0_i64; n];

        for i in 0..m {
            for j in 0..n {
                let val = grid[i][j] as i64;
                h[i] += val;
                v[j] += val;
                p += val;

            }
        }

        let mut s = 0;
        for i in 0..m {
            s += h[i];
            if s * 2 == p {
                return true;
            }
        }

        s = 0;
        for j in 0..n {
            s += v[j];
            if s * 2 == p {
                return true;
            }
        }

        false
    }
}
