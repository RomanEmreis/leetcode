/*
  1727. Largest Submatrix With Rearrangements
  
  You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of the matrix in any order.
  
  Return the area of the largest submatrix within matrix where every element of the submatrix is 1 after reordering the columns optimally.
  
  Example 1:
  Input: matrix = [[0,0,1],[1,1,1],[1,0,1]]
  Output: 4
  Explanation: You can rearrange the columns as shown above.
  The largest submatrix of 1s, in bold, has an area of 4.
  
  Example 2:
  Input: matrix = [[1,0,1,0,1]]
  Output: 3
  Explanation: You can rearrange the columns as shown above.
  The largest submatrix of 1s, in bold, has an area of 3.
  
  Example 3:
  Input: matrix = [[1,1,0],[1,0,1]]
  Output: 2
  Explanation: Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.
*/
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix[0].len();
        
        let mut res = 0;
        let mut hist = vec![0; n];

        matrix.into_iter().for_each(|row| {
            for i in 0..n {
                hist[i] = if row[i] == 0 { 0 } else { hist[i] + 1 };
            }

            let mut sorted = hist.clone();
            sorted.sort_unstable();

            for i in 0..n {
                res = res.max(sorted[i] * (n - i) as i32);
            }
        });

        res
    }
}
