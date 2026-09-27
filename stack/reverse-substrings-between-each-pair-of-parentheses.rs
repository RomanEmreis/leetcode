/*
  1190. Reverse Substrings Between Each Pair of Parentheses
  
  You are given a string s that consists of lower case English letters and brackets.
  
  Reverse the strings in each pair of matching parentheses, starting from the innermost one.
  
  Your result should not contain any brackets.
  
  Example 1:
  Input: s = "(abcd)"
  Output: "dcba"
  
  Example 2:
  Input: s = "(u(love)i)"
  Output: "iloveu"
  Explanation: The substring "love" is reversed first, then the whole string is reversed.
  
  Example 3:
  Input: s = "(ed(et(oc))el)"
  Output: "leetcode"
  Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.
*/
use std::collections::VecDeque;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let n = s.len();
        let b = s.into_bytes();
        let mut st = Vec::with_capacity(n);
        let mut q = VecDeque::with_capacity(n);

        for &c in &b {
            if c != b')' {
                st.push(c);
            } else {
                while let Some(&t) = st.last() && t != b'(' {
                    q.push_back(t);
                    _ = st.pop();
                }

                if !st.is_empty() {
                    _ = st.pop();
                }

                while let Some(t) = q.pop_front() {
                    st.push(t);
                }
            }
        }

        st.into_iter().map(|c| c as char).collect()
    }
}
