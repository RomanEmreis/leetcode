/*
  You are given an n x n integer matrix. You can do the following operation any number of times:
  
  Choose any two adjacent elements of matrix and multiply each of them by -1.
  Two elements are considered adjacent if and only if they share a border.
  
  Your goal is to maximize the summation of the matrix's elements. Return the maximum sum of the matrix's elements using the operation mentioned above.
  
  Example 1:
  Input: matrix = [[1,-1],[-1,1]]
  Output: 4
  Explanation: We can follow the following steps to reach sum equals 4:
  - Multiply the 2 elements in the first row by -1.
  - Multiply the 2 elements in the first column by -1.
*/
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut min = i64::MAX;
        let mut neg_cnt = 0;
        let mut res = 0;

        let n = matrix.len();
        
        for i in 0..n {
            for j in 0..n {
                let num = matrix[i][j];
                let abs = num.abs() as i64;
                
                res += abs;
                min = min.min(abs);

                if num < 0 {
                    neg_cnt += 1;
                }
            }
        }

        if neg_cnt % 2 == 0 {
            res
        } else {
            res - (2 * min)
        }
    }
}
