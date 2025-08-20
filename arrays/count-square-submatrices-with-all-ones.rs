/*
  1277. Count Square Submatrices with All Ones
  
  Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
  
  Example 1:
  Input: matrix =
  [
    [0,1,1,1],
    [1,1,1,1],
    [0,1,1,1]
  ]
  Output: 15
  Explanation: 
  There are 10 squares of side 1.
  There are 4 squares of side 2.
  There is  1 square of side 3.
  Total number of squares = 10 + 4 + 1 = 15.
  
  Example 2:
  Input: matrix = 
  [
    [1,0,1],
    [1,1,0],
    [1,1,0]
  ]
  Output: 7
  Explanation: 
  There are 6 squares of side 1.  
  There is 1 square of side 2. 
  Total number of squares = 6 + 1 = 7.
*/
impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] = 1 + [matrix[i - 1][j], matrix[i - 1][j - 1], matrix[i][j - 1]]
                        .iter()
                        .min()
                        .unwrap();
                } 
                result += matrix[i][j];
            }
        }

        result
    }
}
