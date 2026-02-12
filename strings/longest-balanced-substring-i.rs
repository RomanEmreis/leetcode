/*
  3713. Longest Balanced Substring I
  
  You are given a string s consisting of lowercase English letters.
  A of s is called balanced if all distinct characters in the substring appear the same number of times.
  
  Return the length of the longest balanced substring of s.
  
  Example 1:
  Input: s = "abbac"
  Output: 4
  Explanation:
  The longest balanced substring is "abba" because both distinct characters 'a' and 'b' each appear exactly 2 times.
  
  Example 2:
  Input: s = "zzabccy"
  Output: 4
  Explanation:
  The longest balanced substring is "zabc" because the distinct characters 'z', 'a', 'b', and 'c' each appear exactly 1 time.​​​​​​​
  
  Example 3:
  Input: s = "aba"
  Output: 2
  Explanation:
  ​​​​​​​One of the longest balanced substrings is "ab" because both distinct characters 'a' and 'b' each appear exactly 1 time. Another longest balanced substring is "ba".
*/
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let n = s.len();
        let b = s.into_bytes();

        let mut res = 0;

        for i in 0..n {
            let mut cnt = [0; 26];
            let mut v = 0;
            let mut max = 0;

            for j in i..n {
                let c = (b[j] - b'a') as usize;
                cnt[c] += 1;

                if cnt[c] == 1 {
                    v += 1;
                }

                max = max.max(cnt[c]);

                if max * v == j - i + 1 {
                    res = res.max(j - i + 1);
                }
            }
        }

        res as i32
    }
}
