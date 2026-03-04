/*
  1582. Special Positions in a Binary Matrix
  
  Given an m x n binary matrix mat, return the number of special positions in mat.
  A position (i, j) is called special if mat[i][j] == 1 and
  all other elements in row i and column j are 0 (rows and columns are 0-indexed).
  
  Example 1:
  Input: mat = [[1,0,0],[0,0,1],[1,0,0]]
  Output: 1
  Explanation: (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.
  
  Example 2:
  Input: mat = [[1,0,0],[0,1,0],[0,0,1]]
  Output: 3
  Explanation: (0, 0), (1, 1) and (2, 2) are special positions.
*/
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut res = 0;
        let mut row = vec![0; m];
        let mut col = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && row[i] == 1 && col[j] == 1 {
                    res += 1;
                }
            }
        }

        res
    }
}
