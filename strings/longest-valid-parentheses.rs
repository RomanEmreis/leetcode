/*
  32. Longest Valid Parentheses
  
  Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses substring.
  
  Example 1:
  Input: s = "(()"
  Output: 2
  Explanation: The longest valid parentheses substring is "()".
  
  Example 2:
  Input: s = ")()())"
  Output: 4
  Explanation: The longest valid parentheses substring is "()()".
  
  Example 3:
  Input: s = ""
  Output: 0
*/
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut open = 0;
        let mut close = 0;
        let mut best = 0;

        for &b in bytes {
            if b == b'(' {
                open += 1;
            } else {
                close += 1;
            }

            if open == close {
                best = best.max(2 * close);
            } else if close > open {
                open = 0;
                close = 0;
            }
        }

        open = 0;
        close = 0;

        for &b in bytes.iter().rev() {
            if b == b'(' {
                open += 1;
            } else {
                close += 1;
            }

            if open == close {
                best = best.max(2 * open);
            } else if open > close {
                open = 0;
                close = 0;
            }
        }

        best
    }
}
