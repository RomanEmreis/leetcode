/*
  1358. Number of Substrings Containing All Three Characters
  
  Given a string s consisting only of characters a, b and c.
  
  Return the number of substrings containing at least one occurrence of all these characters a, b and c.
   
  Example 1:
  Input: s = "abcabc"
  Output: 10
  Explanation: The substrings containing at least one occurrence of the characters 
  a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again). 
  
  Example 2:
  Input: s = "aaacb"
  Output: 3
  Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb". 
  
  Example 3:
  Input: s = "abc"
  Output: 1
*/
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let b = s.into_bytes();
        let mut dp: [i32; 3] = [-1, -1, -1];
        let mut res = 0;
        for i in 0..n {
            dp[(b[i] - b'a') as usize] = i as i32;
            res += dp[0].min(dp[1].min(dp[2])) + 1;
        }
        res
    }
}
