/*
  1262. Greatest Sum Divisible by Three
  
  Given an integer array nums, return the maximum possible sum of elements of the array such that it is divisible by three.
  
  Example 1:
  Input: nums = [3,6,5,1,8]
  Output: 18
  Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
  
  Example 2:
  Input: nums = [4]
  Output: 0
  Explanation: Since 4 is not divisible by 3, do not pick any number.
  
  Example 3:
  Input: nums = [1,2,3,4,4]
  Output: 12
  Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
*/
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = [0, 0, 0];
        for num in nums.iter() {
            for sum in dp.clone().iter() {
                let x = sum + num;
                let i = (x % 3) as usize;
                dp[i] = dp[i].max(x);
            }
        }
        dp[0]
    }
}
