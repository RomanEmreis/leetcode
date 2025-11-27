/*
  3381. Maximum Subarray Sum With Length Divisible by K
  
  You are given an array of integers nums and an integer k.
  
  Return the maximum sum of a subarray of nums, such that the size of the subarray is divisible by k.
  
  Example 1:
  Input: nums = [1,2], k = 1
  Output: 3
  Explanation:
  The subarray [1, 2] with sum 3 has length equal to 2 which is divisible by 1.
  
  Example 2:
  Input: nums = [-1,-2,-3,-4,-5], k = 4
  Output: -10
  Explanation:
  The maximum sum subarray is [-1, -2, -3, -4] which has length equal to 4 which is divisible by 4.
  
  Example 3:
  Input: nums = [-5,1,2,-3,4], k = 2
  Output: 4
  Explanation:
  The maximum sum subarray is [1, 2, -3, 4] which has length equal to 4 which is divisible by 2.
*/
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = nums.len();
        let mut res = i64::MIN;
        let mut prefix = 0;
        let mut min = vec![i64::MAX / 2; k];
        min[k - 1] = 0;
        for i in 0..n {
            prefix += nums[i] as i64;
            let j = i % k;
            res = res.max(prefix - min[j]);
            min[j] = prefix.min(min[j]);
        }
        res
    }
}
