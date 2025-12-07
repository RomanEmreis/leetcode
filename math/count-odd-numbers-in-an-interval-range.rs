/*
  1523. Count Odd Numbers in an Interval Range
  
  Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).
  
  Example 1:
  Input: low = 3, high = 7
  Output: 3
  Explanation: The odd numbers between 3 and 7 are [3,5,7].
  
  Example 2:
  Input: low = 8, high = 10
  Output: 1
  Explanation: The odd numbers between 8 and 10 are [9].
*/
impl Solution {
    pub fn count_odds(mut low: i32, mut high: i32) -> i32 {
        let mut res = 0;

        if low % 2 != 0 {
            low += 1;
            res += 1;
        }
        if high % 2 != 0 {
            high -= 1;
            res += 1;
        }

        res + (high - low) / 2
    }
}
