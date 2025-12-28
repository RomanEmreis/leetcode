/*
  1351. Count Negative Numbers in a Sorted Matrix
  
  Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise, 
  return the number of negative numbers in grid.
  
  Example 1:
  Input: grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
  Output: 8
  Explanation: There are 8 negatives number in the matrix.
  
  Example 2:
  Input: grid = [[3,2],[1,0]]
  Output: 0
*/
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut res = 0;
        let mut i = m - 1;
        let mut j = 0;

        while i >= 0 && j < n {
            if grid[i][j] < 0 {
                res += n - j;
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            } else {
                j += 1;
            }
        }

        res as i32
    }
}
