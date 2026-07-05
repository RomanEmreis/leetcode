/*
  1301. Number of Paths with Max Score
  
  You are given a square board of characters. You can move on the board starting at the bottom right square marked with the character 'S'.
  You need to reach the top left square marked with the character 'E'. The rest of the squares are labeled either with a numeric character 1, 2, ..., 9 or
  with an obstacle 'X'. In one move you can go up, left or up-left (diagonally) only if there is no obstacle there.
  
  Return a list of two integers: the first integer is the maximum sum of numeric characters you can collect, 
  and the second is the number of such paths that you can take to get that maximum sum, taken modulo 10^9 + 7.
  
  In case there is no path, return [0, 0].
  
  Example 1:
  Input: board = ["E23","2X2","12S"]
  Output: [7,1]
  
  Example 2:
  Input: board = ["E12","1X1","21S"]
  Output: [4,2]
  
  Example 3:
  Input: board = ["E11","XXX","11S"]
  Output: [0,0]
*/
const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let n = board.len();
        let grid: Vec<&[u8]> = board.iter().map(|r| r.as_bytes()).collect();

        let mut score = vec![-1_i32; n];
        let mut count = vec![0_u64; n];

        for i in (0..n).rev() {
            let mut diag_s = -1_i32;
            let mut diag_c = 0_u64;

            for j in (0..n).rev() {
                let down_s = score[j];
                let down_c = count[j];

                let cell = grid[i][j];
                if cell == b'X' {
                    score[j] = -1;
                    count[j] = 0;
                } else if i == n - 1 && j == n - 1 {
                    score[j] = 0;
                    count[j] = 1;
                } else {
                    let (right_s, right_c) = if j + 1 < n {
                        (score[j + 1], count[j + 1])
                    } else {
                        (-1, 0)
                    };

                    let best = down_s.max(right_s).max(diag_s);
                    if best < 0 {
                        score[j] = -1;
                        count[j] = 0;
                    } else {
                        let mut c = 0_u64;
                        if down_s == best { c += down_c; }
                        if right_s == best { c += right_c; }
                        if diag_s == best { c += diag_c; }
                        let gain = if cell.is_ascii_digit() { i32::from(cell - b'0') } else { 0 };
                        score[j] = best + gain;
                        count[j] = c % MOD;
                    }
                }

                diag_s = down_s;
                diag_c = down_c;
            }
        }

        if score[0] < 0 { vec![0, 0] } else { vec![score[0], count[0] as i32] }
    }
}
