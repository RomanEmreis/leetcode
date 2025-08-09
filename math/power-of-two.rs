/*
  231. Power of Two
  
  Given an integer n, return true if it is a power of two. Otherwise, return false.
  
  An integer n is a power of two, if there exists an integer x such that n == 2x.
  
  Example 1:
  Input: n = 1
  Output: true
  Explanation: 20 = 1
  
  Example 2:
  Input: n = 16
  Output: true
  Explanation: 24 = 16
  
  Example 3:
  Input: n = 3
  Output: false
*/
impl Solution {
    const MAX: i32 = 1_073_741_824;
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && Self::MAX % n == 0
    }
}
