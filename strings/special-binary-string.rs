/*
  761. Special Binary String
  
  Special binary strings are binary strings with the following two properties:
      The number of 0's is equal to the number of 1's.
      Every prefix of the binary string has at least as many 1's as 0's.
  
  You are given a special binary string s.
  
  A move consists of choosing two consecutive, non-empty, special substrings of s, and swapping them.
  Two strings are consecutive if the last character of the first string is exactly one index before the first character of the second string.
  
  Return the lexicographically largest resulting string possible after applying the mentioned operations on the string.
  
  Example 1:
  Input: s = "11011000"
  Output: "11100100"
  Explanation: The strings "10" [occuring at s[1]] and "1100" [at s[3]] are swapped.
  This is the lexicographically largest string possible after some number of swaps.
  
  Example 2:
  Input: s = "10"
  Output: "10"
*/
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut specials = Vec::new();
        let mut count = 0;

        let bytes = s.as_bytes();
        let mut i = 0;
        for j in 0..bytes.len() {
            count += if bytes[j] == b'1' { 1 } else { -1 };
            if count == 0 {
                let inner = Self::make_largest_special(s[i + 1..j].to_owned());
                specials.push(format!("1{inner}0"));
                i = j + 1;
            }
        }

        specials.sort_by(|a, b| b.cmp(a));
        specials.join("")
    }
}
