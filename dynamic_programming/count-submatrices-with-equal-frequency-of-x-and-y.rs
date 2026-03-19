/*
  3212. Count Submatrices With Equal Frequency of X and Y
  
  Given a 2D character matrix grid, where grid[i][j] is either 'X', 'Y', or '.', return the number of that contain:
      grid[0][0]
      an equal frequency of 'X' and 'Y'.
      at least one 'X'.
  
  Example 1:
  Input: grid = [["X","Y","."],["Y",".","."]]
  Output: 3
  Explanation:
  
  Example 2:
  Input: grid = [["X","X"],["X","Y"]]
  Output: 0
  Explanation:
  No submatrix has an equal frequency of 'X' and 'Y'.
  
  Example 3:
  Input: grid = [[".","."],[".","."]]
  Output: 0
  Explanation:
  No submatrix has at least one 'X'.
*/
impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len();
        let mut diff = vec![0i32; n];
        let mut x_cnt = vec![0i32; n];
        let mut res = 0i32;

        for row in &grid {
            let mut pd = 0i32;
            let mut px = 0i32;
            for j in 0..n {
                diff[j] += match row[j] {
                    c if c == 'X' => 1,
                    c if c == 'Y' => -1,
                    _ => 0,
                };
                
                if row[j] == 'X' {
                    x_cnt[j] += 1;
                }
                
                pd += diff[j];
                px += x_cnt[j];

                if pd == 0 && px > 0 {
                    res += 1;
                }
            }
        }

        res
    }
}
