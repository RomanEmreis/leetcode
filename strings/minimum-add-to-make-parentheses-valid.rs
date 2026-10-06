/*
  921. Minimum Add to Make Parentheses Valid
  
  A parentheses string is valid if and only if:
      It is the empty string,
      It can be written as AB (A concatenated with B), where A and B are valid strings, or
      It can be written as (A), where A is a valid string.
  
  You are given a parentheses string s. In one move, you can insert a parenthesis at any position of the string.
      For example, if s = "()))", you can insert an opening parenthesis to be "(()))" or a closing parenthesis to be "())))".
  
  Return the minimum number of moves required to make s valid.
  
  Example 1:
  Input: s = "())"
  Output: 1
  
  Example 2:
  Input: s = "((("
  Output: 3
*/
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut st = Vec::new();
        
        for ch in s.bytes() {
            if ch == b')' && let Some(l) = st.last() && l == b'(' {
                st.pop();
            } else {
                st.push(ch.into());
            }
        }

        st.len() as i32
    }
}
