/*
  2014. Longest Subsequence Repeated k Times
  
  You are given a string s of length n, and an integer k. You are tasked to find the longest subsequence repeated k times in string s.
  A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
  A subsequence seq is repeated k times in the string s if seq * k is a subsequence of s, where seq * k represents a string constructed by concatenating seq k times.
  
  For example, "bba" is repeated 2 times in the string "bababcba", because the string "bbabba", constructed by concatenating "bba" 2 times, is a subsequence of the string "bababcba".
  Return the longest subsequence repeated k times in string s. If multiple such subsequences are found, return the lexicographically largest one. If there is no such subsequence, return an empty string.
  
  Example 1:
  Input: s = "letsleetcode", k = 2
  Output: "let"
  Explanation: There are two longest subsequences repeated 2 times: "let" and "ete".
  "let" is the lexicographically largest one.
  
  Example 2:
  Input: s = "bb", k = 2
  Output: "b"
  Explanation: The longest subsequence repeated 2 times is "b".
  
  Example 3:
  Input: s = "ab", k = 2
  Output: ""
  Explanation: There is no subsequence repeated 2 times. Empty string is returned.
*/
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        fn check(s: &[u8], t: &[u8], mut k: i32) -> bool {
            let mut i = 0;
            let ln = t.len();
            for &ch in s {
                if ch == t[i] {
                    i += 1;
                    if i == ln {
                        k -= 1;
                        if k == 0 {
                            return true;
                        }
                        i = 0;
                    }
                }
            }
            false
        }

        let bytes = s.as_bytes();
        let mut freq = [0; 26];
        for &ch in bytes {
            freq[(ch - b'a') as usize] += 1;
        }

        let mut counts = Vec::new();
        for ch in 'a'..='z' {
            if freq[(ch as u8 - b'a') as usize] >= k {
                counts.push(ch);
            }
        }

        let mut q = std::collections::VecDeque::new();
        q.push_back(String::new());

        let mut result = String::new();
        while !q.is_empty() {
            let mut curr = q.pop_front().unwrap();
            for &ch in &counts {
                let mut next = {
                    let mut s = curr.clone();
                    s.push(ch);
                    s
                };
                if check(bytes, next.as_bytes(), k) {
                    q.push_back(next.clone());
                    result = next;
                }               
            }
        }
        result
    }
}
