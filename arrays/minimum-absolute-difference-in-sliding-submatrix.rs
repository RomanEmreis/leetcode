/*
  3567. Minimum Absolute Difference in Sliding Submatrix
  
  You are given an m x n integer matrix grid and an integer k.
  For every contiguous k x k submatrix of grid, compute the minimum absolute difference between any two distinct values within that submatrix.
  
  Return a 2D array ans of size (m - k + 1) x (n - k + 1), where ans[i][j] is the minimum absolute difference in the submatrix whose top-left corner is (i, j) in grid.
  
  Note: If all elements in the submatrix have the same value, the answer will be 0.
  A submatrix (x1, y1, x2, y2) is a matrix that is formed by choosing all cells matrix[x][y] where x1 <= x <= x2 and y1 <= y <= y2.
  
  Example 1:
  Input: grid = [[1,8],[3,-2]], k = 2
  Output: [[2]]
  Explanation:
      There is only one possible k x k submatrix: [[1, 8], [3, -2]].
      Distinct values in the submatrix are [1, 8, 3, -2].
      The minimum absolute difference in the submatrix is |1 - 3| = 2. Thus, the answer is [[2]].
  
  Example 2:
  Input: grid = [[3,-1]], k = 1
  Output: [[0,0]]
  Explanation:
      Both k x k submatrix has only one distinct element.
      Thus, the answer is [[0, 0]].
  
  Example 3:
  Input: grid = [[1,-2,3],[2,3,5]], k = 2
  Output: [[1,2]]
  Explanation:
      There are two possible k × k submatrix:
          Starting at (0, 0): [[1, -2], [2, 3]].
              Distinct values in the submatrix are [1, -2, 2, 3].
              The minimum absolute difference in the submatrix is |1 - 2| = 1.
          Starting at (0, 1): [[-2, 3], [3, 5]].
              Distinct values in the submatrix are [-2, 3, 5].
              The minimum absolute difference in the submatrix is |3 - 5| = 2.
      Thus, the answer is [[1, 2]].
*/
impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        
        let mut res = vec![vec![0; n - k + 1]; m - k + 1];

        for i in 0..=m - k {
            for j in 0..=n - k {
                let mut nums = Vec::with_capacity(k * k);
                for x in i..i + k {
                    for y in j..j + k {
                        nums.push(grid[x][y]);
                    } 
                }

                nums.sort_unstable();

                let mut diff = i32::MAX;
                for x in 1..nums.len() {
                    if nums[x] != nums[x - 1] {
                        diff = diff.min((nums[x] - nums[x - 1]).abs());
                    }
                }

                res[i][j] = if diff == i32::MAX { 0 } else { diff };
            }
        }

        res
    }
}
