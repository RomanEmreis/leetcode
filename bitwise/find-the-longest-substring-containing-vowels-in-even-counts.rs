/*
  Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
  
  Example 1:
    Input: s = "eleetminicoworoep"
    Output: 13
    Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
*/
use std::collections::HashMap;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let (mut result, mut mask) = (0, 0);
        let mut seen = vec![-1; 1 << 5];
        seen[0] = 0;

        for (i, ch) in s.chars().enumerate() {
            match ch {
                'a' => mask ^= 1 << 0,
                'e' => mask ^= 1 << 1,
                'i' => mask ^= 1 << 2,
                'o' => mask ^= 1 << 3,
                'u' => mask ^= 1 << 4,
                _ => {}
            }

            if seen[mask] >= 0 {
                result = result.max(i + 1 - seen[mask] as usize);
            } else {
                seen[mask] = i as i32 + 1;
            }
        }

        result as i32
    }
}
