/*
  1886. Determine Whether Matrix Can Be Obtained By Rotation
  
  Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to
  target by rotating mat in 90-degree increments, or false otherwise.
  
  Example 1:
  Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
  Output: true
  Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.
  
  Example 2:
  Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
  Output: false
  Explanation: It is impossible to make mat equal to target by rotating mat.
  
  Example 3:
  Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
  Output: true
  Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.
*/
impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        for _ in 0..4 {
            if mat == target {
                return true;
            }
            rotate(&mut mat, n);
        }
        false
    }
}

#[inline(always)]
fn rotate(mat: &mut Vec<Vec<i32>>, n: usize) {
    let x = n / 2;
    let y = (n + 1) / 2;
    for i in 0..x {
        for j in 0..y {
            let t = mat[i][j];
            mat[i][j] = mat[n - 1 - j][i];
            mat[n - 1 - j][i] = mat[n - 1 - i][n - 1 - j];
            mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
            mat[j][n - 1 - i] = t;
        }
    }
}
