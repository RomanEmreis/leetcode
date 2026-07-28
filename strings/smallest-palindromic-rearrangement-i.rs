/*
  3517. Smallest Palindromic Rearrangement I
  
  You are given a string s.
  
  Return the palindromic of s.
  
  Example 1:
  Input: s = "z"
  Output: "z"
  Explanation:
  A string of only one character is already the lexicographically smallest palindrome.
  
  Example 2:
  Input: s = "babab"
  Output: "abbba"
  Explanation:
  Rearranging "babab" → "abbba" gives the smallest lexicographic palindrome.
  
  Example 3:
  Input: s = "daccad"
  Output: "acddca"
  Explanation:
  Rearranging "daccad" → "acddca" gives the smallest lexicographic palindrome.
*/
impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut freq = [0usize; 26];

        for &byte in s.as_bytes() {
            freq[(byte - b'a') as usize] += 1;
        }

        let mut res = s.into_bytes();
        let n = res.len();
        let m = n / 2;
        let mut l = 0usize;
        let mut r = n - 1;

        for i in 0..26 {
            let byte = b'a' + i as u8;
            let count = freq[i];

            for _ in 0..count / 2 {
                res[l] = byte;
                res[r] = byte;
                l += 1;
                r -= 1;
            }

            if count & 1 != 0 {
                res[m] = byte;
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}
