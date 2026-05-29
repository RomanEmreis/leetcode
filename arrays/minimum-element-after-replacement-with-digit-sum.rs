/*
  3300. Minimum Element After Replacement With Digit Sum
  
  You are given an integer array nums.
  You replace each element in nums with the sum of its digits.
  
  Return the minimum element in nums after all replacements.
  
  Example 1:
  Input: nums = [10,12,13,14]
  Output: 1
  Explanation:
  nums becomes [1, 3, 4, 5] after all replacements, with minimum element 1.
  
  Example 2:
  Input: nums = [1,2,3,4]
  Output: 1
  Explanation:
  nums becomes [1, 2, 3, 4] after all replacements, with minimum element 1.
  
  Example 3:
  Input: nums = [999,19,199]
  Output: 10
  Explanation:
  nums becomes [27, 10, 19] after all replacements, with minimum element 10.
*/
use std::iter::successors;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut n| {
                successors(Some(n), |&x| (x >= 10).then_some(x / 10))
                    .map(|x| x % 10)
                    .sum::<i32>()
            })
        .min()
        .unwrap()
    }
}
