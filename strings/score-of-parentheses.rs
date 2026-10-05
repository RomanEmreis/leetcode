/*
  856. Score of Parentheses
  
  Given a balanced parentheses string s, return the score of the string.
  The score of a balanced parentheses string is based on the following rule:
      "()" has score 1.
      AB has score A + B, where A and B are balanced parentheses strings.
      (A) has score 2 * A, where A is a balanced parentheses string.
  
  Example 1:
  Input: s = "()"
  Output: 1
  
  Example 2:
  Input: s = "(())"
  Output: 2
  
  Example 3:
  Input: s = "()()"
  Output: 2
*/
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut score = 0;
        let mut depth = 0;
        let mut open = false;

        for byte in s.bytes() {
            if byte == b'(' {
                depth += 1;
                open = true;
            } else {
                depth -= 1;

                if open {
                    score += 1 << depth;
                }

                open = false;
            }
        }

        score
    }
}
