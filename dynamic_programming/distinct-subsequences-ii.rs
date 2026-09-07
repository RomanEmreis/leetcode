/*
  940. Distinct Subsequences II
  
  Given a string s, return the number of distinct non-empty subsequences of s. Since the answer may be very large, return it modulo 109 + 7.
  A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) 
  of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not.
  
  Example 1:
  Input: s = "abc"
  Output: 7
  Explanation: The 7 distinct subsequences are "a", "b", "c", "ab", "ac", "bc", and "abc".
  
  Example 2:
  Input: s = "aba"
  Output: 6
  Explanation: The 6 distinct subsequences are "a", "b", "ab", "aa", "ba", and "aba".
  
  Example 3:
  Input: s = "aaa"
  Output: 3
  Explanation: The 3 distinct subsequences are "a", "aa" and "aaa".
*/
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut last = [0i32; 26];
        let mut total = 1i32;

        for byte in s.bytes() {
            let index = (byte - b'a') as usize;
            let old_total = total;

            let mut next = total * 2 - last[index];

            if next >= MOD {
                next -= MOD;
            } else if next < 0 {
                next += MOD;
            }

            last[index] = old_total;
            total = next;
        }

        if total == 0 {
            MOD - 1
        } else {
            total - 1
        }
    }
}
