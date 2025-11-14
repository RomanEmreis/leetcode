/*
  2536. Increment Submatrices by One
  
  You are given a positive integer n, indicating that we initially have an n x n 0-indexed integer matrix mat filled with zeroes.
  You are also given a 2D integer array query. For each query[i] = [row1i, col1i, row2i, col2i], you should do the following operation:
      Add 1 to every element in the submatrix with the top left corner (row1i, col1i) and the bottom right corner (row2i, col2i). That is, add 1 to mat[x][y] for all row1i <= x <= row2i and col1i <= y <= col2i.
  
  Return the matrix mat after performing every query.
  
  Example 1:
  Input: n = 3, queries = [[1,1,2,2],[0,0,1,1]]
  Output: [[1,1,0],[1,2,1],[0,1,1]]
  Explanation: The diagram above shows the initial matrix, the matrix after the first query, and the matrix after the second query.
  - In the first query, we add 1 to every element in the submatrix with the top left corner (1, 1) and bottom right corner (2, 2).
  - In the second query, we add 1 to every element in the submatrix with the top left corner (0, 0) and bottom right corner (1, 1).
  
  Example 2:
  Input: n = 2, queries = [[0,0,1,1]]
  Output: [[1,1],[1,1]]
  Explanation: The diagram above shows the initial matrix and the matrix after the first query.
  - In the first query we add 1 to every element in the matrix.
*/
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut prefix = vec![vec![0; n + 1]; n + 1];
        for q in queries.into_iter() {
            let (r1, c1, r2, c2) = (
                q[0] as usize, 
                q[1] as usize, 
                q[2] as usize, 
                q[3] as usize
            );
            prefix[r1][c1] += 1;
            prefix[r2 + 1][c1] -= 1;
            prefix[r1][c2 + 1] -= 1;
            prefix[r2 + 1][c2 + 1] += 1;
        }

        let mut res = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let x1 = if i == 0 { 0 } else { res[i - 1][j] };
                let x2 = if j == 0 { 0 } else { res[i][j - 1] };
                let x3 = if i == 0 || j == 0 { 0 } else { res[i - 1][j - 1] };
                res[i][j] = prefix[i][j] + x1 + x2 - x3;
            }
        }
        res
    }
}
