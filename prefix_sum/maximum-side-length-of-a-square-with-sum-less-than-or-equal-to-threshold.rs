/*
  1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
  
  Given a m x n matrix mat and an integer threshold, return the maximum side-length of a square with a sum less than or equal to threshold or return 0 if there is no such square.
  
  Example 1:
  Input: mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
  Output: 2
  Explanation: The maximum side length of square with sum less than 4 is 2 as shown.
  
  Example 2:
  Input: mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
  Output: 0
*/
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut prefix = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                prefix[i + 1][j + 1] = mat[i][j]
                    + prefix[i][j + 1]
                    + prefix[i + 1][j]
                    - prefix[i][j];
            }
        }

        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                for l in res..std::cmp::min(m - i, n - j) {
                    if sum(&prefix, i, j, i + l, j + l) > threshold {
                        break;
                    }
                    res = res.max(l + 1);
                }
            }
        }

        res as i32
    }
}

#[inline(always)]
fn sum(
    prefix: &[Vec<i32>], 
    r1: usize, 
    c1: usize, 
    r2: usize, 
    c2: usize
) -> i32 {
    prefix[r2 + 1][c2 + 1] 
        - prefix[r1][c2 + 1] 
        - prefix[r2 + 1][c1] 
        + prefix[r1][c1]
}
