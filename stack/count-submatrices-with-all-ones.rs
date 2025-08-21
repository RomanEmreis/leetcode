/*
  1504. Count Submatrices With All Ones
  
  Given an m x n binary matrix mat, return the number of submatrices that have all ones.
  
  Example 1:
  Input: mat = [[1,0,1],[1,1,0],[1,1,0]]
  Output: 13
  Explanation: 
  There are 6 rectangles of side 1x1.
  There are 2 rectangles of side 1x2.
  There are 3 rectangles of side 2x1.
  There is 1 rectangle of side 2x2. 
  There is 1 rectangle of side 3x1.
  Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.
  
  Example 2:
  Input: mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
  Output: 24
  Explanation: 
  There are 8 rectangles of side 1x1.
  There are 5 rectangles of side 1x2.
  There are 2 rectangles of side 1x3. 
  There are 4 rectangles of side 2x1.
  There are 2 rectangles of side 2x2. 
  There are 2 rectangles of side 3x1. 
  There is 1 rectangle of side 3x2. 
  Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.
*/
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut hist = vec![0; mat[0].len()];
        for row in mat {
            for i in 0..row.len() {
                hist[i] = if row[i] == 0 { 0 } else { hist[i] + 1 };
            }
            result += Self::calc(&hist);
        }
        result
    }

    fn calc(hist: &Vec<i32>) -> i32 {
        let n = hist.len();
        let mut sub = vec![0; n];
        let mut st = std::collections::VecDeque::with_capacity(n);
        for i in 0..n {
            while let Some(front) = st.front() && hist[*front] >= hist[i] {
                st.pop_front();
            }
            if let Some(front) = st.front() {
                sub[i] = sub[*front] + hist[i] * (i - front) as i32;
            } else {
                sub[i] = hist[i] * (i + 1) as i32;
            }
            st.push_front(i);
        }
        sub.iter().sum()
    }
}
