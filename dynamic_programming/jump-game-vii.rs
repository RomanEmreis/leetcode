/*
  1871. Jump Game VII
  
  You are given a 0-indexed binary string s and two integers minJump and maxJump. 
  In the beginning, you are standing at index 0, which is equal to '0'. You can move from index i to index j if the following conditions are fulfilled:
      i + minJump <= j <= min(i + maxJump, s.length - 1), and
      s[j] == '0'.
  
  Return true if you can reach index s.length - 1 in s, or false otherwise.
  
  Example 1:
  Input: s = "011010", minJump = 2, maxJump = 3
  Output: true
  Explanation:
  In the first step, move from index 0 to index 3. 
  In the second step, move from index 3 to index 5.
  
  Example 2:
  Input: s = "01101110", minJump = 2, maxJump = 3
  Output: false
*/
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let n = s.len();
        let b = s.into_bytes();
        let min = min_jump as usize;
        let max = max_jump as usize;

        let mut cnt = 0;
        let mut dp = vec![false; n];
        dp[0] = true;

        for i in min..n {
            if dp[i - min] {
                cnt += 1;
            }

            if i > max && dp[i - max - 1] {
                cnt -= 1;
            }

            dp[i] = cnt > 0 && b[i] == b'0';
        }

        dp[n - 1]
    }
}
