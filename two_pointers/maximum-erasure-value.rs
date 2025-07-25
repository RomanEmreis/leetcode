/*
  1695. Maximum Erasure Value
  
  You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.
  Return the maximum score you can get by erasing exactly one subarray.
  
  An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).
  
  Example 1:
  Input: nums = [4,2,4,5,6]
  Output: 17
  Explanation: The optimal subarray here is [2,4,5,6].
  
  Example 2:
  Input: nums = [5,2,1,2,5,2,1,2,5]
  Output: 8
  Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
*/
use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut seen = HashSet::with_capacity(n);
        let mut result = 0;
        let mut sum = 0;
        let mut i = 0;
        nums.iter().for_each(|&num| {
            while !seen.insert(num) {
                sum -= nums[i];
                seen.remove(&nums[i]);
                i += 1;
            }
            sum += num;
            result = result.max(sum);
        });
        result
    }
}
