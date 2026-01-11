/*
  85. Maximal Rectangle
  
  Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
  
  Example 1:
  Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
  Output: 6
  Explanation: The maximal rectangle is shown in the above picture.
  
  Example 2:
  Input: matrix = [["0"]]
  Output: 0
  
  Example 3:
  Input: matrix = [["1"]]
  Output: 1
*/
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut hist = vec![0; n];
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                hist[j] = if matrix[i][j] == '0' { 0 } else { hist[j] + 1 };
            }
            res = res.max(get_max_area(&hist));
        }
        res
    }
}

fn get_max_area(hist: &Vec<i32>) -> i32 {
    let n = hist.len();
    let mut res = 0;
    let mut st = Vec::with_capacity(n);

    for i in 0..=n {
        while let Some(&j) = st.last() && (i == n || hist[j] > hist[i]) {
            let _ = st.pop();
            let h = hist[j];
            let w = if let Some(j) = st.last() {
                i - j - 1
            } else {
                i
            } as i32;
            res = res.max(h * w);
        }
        st.push(i);
    }

    res
}
