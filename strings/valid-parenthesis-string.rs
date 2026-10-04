/*
  678. Valid Parenthesis String
  
  Given a string s containing only three types of characters: '(', ')' and '*', return true if s is valid.
  The following rules define a valid string:
      Any left parenthesis '(' must have a corresponding right parenthesis ')'.
      Any right parenthesis ')' must have a corresponding left parenthesis '('.
      Left parenthesis '(' must go before the corresponding right parenthesis ')'.
      '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string "".
  
  Example 1:
  Input: s = "()"
  Output: true
  
  Example 2:
  Input: s = "(*)"
  Output: true
  
  Example 3:
  Input: s = "(*))"
  Output: true
  
  Example 4:
  Input: s = "("
  Output: false
*/
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = 0;
        let mut open_max = 0;
        for &ch in s.as_bytes() {
            match ch {
                b'(' => {
                    open += 1;
                    open_max += 1;
                },
                b')' => {
                    open -= 1;
                    open_max -= 1;
                },
                _ => {
                    open -= 1;
                    open_max += 1;
                }
            }

            if open < 0 {
                open = 0;
            }
            if open_max < 0 {
                return false;
            }
        }
        open == 0
    }
}
