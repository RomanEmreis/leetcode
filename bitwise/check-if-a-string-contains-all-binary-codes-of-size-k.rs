/*
  1461. Check If a String Contains All Binary Codes of Size K
  
  Given a binary string s and an integer k, return true if every binary code of length k is a substring of s. Otherwise, return false.
  
  Example 1:
  Input: s = "00110110", k = 2
  Output: true
  Explanation: The binary codes of length 2 are "00", "01", "10" and "11". 
  They can be all found as substrings at indices 0, 1, 3 and 2 respectively.
  
  Example 2:
  Input: s = "0110", k = 1
  Output: true
  Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring. 
  
  Example 3:
  Input: s = "0110", k = 2
  Output: false
  Explanation: The binary code "00" is of length 2 and does not exist in the array.
*/
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = 1 << k;
        let m = s.len();
        if m < n {
            return false;
        }

        let bytes = s.as_bytes();
        let mut used = vec![false; n as usize];
        let mut win: usize = if k == 1 { 0 } else { s[..k - 1].parse().unwrap() };
        for i in k - 1..m {
            win = (win << 1) + (bytes[i] - b'0') as usize;
            win &= n - 1;
            used[win] = true;
        }

        used.into_iter().all(|u| u)
    }
}
