/*
  1614. Maximum Nesting Depth of the Parentheses
  
  Given a valid parentheses string s, return the nesting depth of s. The nesting depth is the maximum number of nested parentheses.
  
  Example 1:
  Input: s = "(1+(2*3)+((8)/4))+1"
  Output: 3
  Explanation:
  Digit 8 is inside of 3 nested parentheses in the string.
  
  Example 2:
  Input: s = "(1)+((2))+(((3)))"
  Output: 3
  Explanation:
  Digit 3 is inside of 3 nested parentheses in the string.
  
  Example 3:
  Input: s = "()(())((()()))"
  Output: 3
*/
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (_, res) = s
            .into_bytes()
            .into_iter()
            .fold((0,0), |(curr, max), b| match b {
                b'(' => {
                    let c = curr + 1;
                    (c, max.max(c))
                },
                b')' => (curr - 1, max),
                _ => (curr, max),
            });
        res
    }
}
