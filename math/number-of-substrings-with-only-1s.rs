/*
  1513. Number of Substrings With Only 1s
  
  Given a binary string s, return the number of substrings with all characters 1's. Since the answer may be too large, return it modulo 109 + 7.
  
  Example 1:
  Input: s = "0110111"
  Output: 9
  Explanation: There are 9 substring in total with only 1's characters.
  "1" -> 5 times.
  "11" -> 3 times.
  "111" -> 1 time.
  
  Example 2:
  Input: s = "101"
  Output: 2
  Explanation: Substring "1" is shown 2 times in s.
  
  Example 3:
  Input: s = "111111"
  Output: 21
  Explanation: Each substring contains only 1's characters.
*/
const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut res = 0;
        let mut l = -1;
        let b = s.as_bytes();
        for i in 0..s.len() {
            if b[i] == b'0' {
                l = i as i32;
            }
            res = (res + i as i32 - l) % MOD;
        }
        res
    }
}
