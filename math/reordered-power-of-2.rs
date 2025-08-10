/*
  869. Reordered Power of 2
  
  You are given an integer n. We reorder the digits in any order (including the original order) such that the leading digit is not zero.
  
  Return true if and only if we can do this so that the resulting number is a power of two.
  
  Example 1:
  Input: n = 1
  Output: true
  
  Example 2:
  Input: n = 10
  Output: false
*/
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        const limit: i32 = 1_000_000_000;
        let target = Self::calc(n);
        let mut i = 1i32;
        while i <= limit {
            if target == Self::calc(i) {
                return true;
            }
            i <<= 1;
        }
        false
    }

    fn calc(mut x: i32) -> [u8; 10] {
        let mut cnt = [0u8; 10];
        while x > 0 {
            cnt[(x % 10) as usize] += 1;
            x /= 10;
        }
        cnt
    }
}
