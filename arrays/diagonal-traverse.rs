/*
  498. Diagonal Traverse
  
  Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
  
  Example 1:
  Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
  Output: [1,2,4,7,5,3,6,8,9]
  
  Example 2:
  Input: mat = [[1,2],[3,4]]
  Output: [1,2,3,4]
*/
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();

        let mut result = vec![0i32; m * n];
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 1;

        for i in 0..result.len() {
            result[i] = mat[r as usize][c as usize];
            r -= d;
            c += d;

            if r as usize == m {
                r = m as i32 - 1;
                c += 2;
                d = -d;
            }
            if c as usize == n {
                c = n as i32 - 1;
                r += 2;
                d = -d;
            }
            if r < 0 {
                r = 0;
                d = -d;
            }
            if c < 0 {
                c = 0;
                d = -d;
            }
        }

        result
    }
}
