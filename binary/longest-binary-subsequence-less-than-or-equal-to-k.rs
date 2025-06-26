/*
  2311. Longest Binary Subsequence Less Than or Equal to K
  
  You are given a binary string s and a positive integer k.
  
  Return the length of the longest subsequence of s that makes up a binary number less than or equal to k.
  
  Note:
  The subsequence can contain leading zeroes.
  The empty string is considered to be equal to 0.
  A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
   
  
  Example 1:
  Input: s = "1001010", k = 5
  Output: 5
  Explanation: The longest subsequence of s that makes up a binary number less than or equal to 5 is "00010", as this number is equal to 2 in decimal.
  Note that "00100" and "00101" are also possible, which are equal to 4 and 5 in decimal, respectively.
  The length of this subsequence is 5, so 5 is returned.
  
  Example 2:
  Input: s = "00101001", k = 1
  Output: 6
  Explanation: "000001" is the longest subsequence of s that makes up a binary number less than or equal to 1, as this number is equal to 1 in decimal.
  The length of this subsequence is 6, so 6 is returned.
*/
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let (mut ones, mut zeros, mut num, mut pow) = (0, 0, 0, 1);
        let bytes = s.as_bytes();
        for ch in bytes {
            if *ch == b'0' {
                zeros += 1;
            }
        }
        let mut i: i32 = (s.len() - 1) as i32;
        while i >= 0 && num + pow <= k {
            if bytes[i as usize] == b'1' {
                ones += 1;
                num += pow;
            }
            pow *= 2;
            i -= 1;
        }
        zeros + ones
    }
}
