/*
  3783. Mirror Distance of an Integer
  
  You are given an integer n.
  Define its mirror distance as: abs(n - reverse(n))​​​​​​​ where reverse(n) is the integer formed by reversing the digits of n.
  
  Return an integer denoting the mirror distance of n​​​​​​​.
  
  abs(x) denotes the absolute value of x.
  
  Example 1:
  Input: n = 25
  Output: 27
  Explanation:
      reverse(25) = 52.
      Thus, the answer is abs(25 - 52) = 27.
  
  Example 2:
  Input: n = 10
  Output: 9
  Explanation:
      reverse(10) = 01 which is 1.
      Thus, the answer is abs(10 - 1) = 9.
  
  Example 3:
  Input: n = 7
  Output: 0
  Explanation:
      reverse(7) = 7.
      Thus, the answer is abs(7 - 7) = 0.
*/
impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut x = n;
        let mut y = 0;
        while x > 0 {
            y = y * 10 + x % 10;
            x /= 10;
        }
        n.abs_diff(y) as i32
    }
}
