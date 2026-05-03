/*
  796. Rotate String
  
  Given two strings s and goal, return true if and only if s can become goal after some number of shifts on s.
  A shift on s consists of moving the leftmost character of s to the rightmost position.
      For example, if s = "abcde", then it will be "bcdea" after one shift.
  
  Example 1:
  Input: s = "abcde", goal = "cdeab"
  Output: true
  
  Example 2:
  Input: s = "abcde", goal = "abced"
  Output: false
*/
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let n = s.len();

        if n != goal.len() {
            return false;
        }
        
        for i in 0..n {
            if goal[i..].bytes().chain(goal[..i].bytes()).eq(s.bytes()) {
                return true;
            }
        }
        
        false
    }
}
