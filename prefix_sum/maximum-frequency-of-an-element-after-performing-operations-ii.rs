/*
  3347. Maximum Frequency of an Element After Performing Operations II
  
  You are given an integer array nums and two integers k and numOperations.
    You must perform an operation numOperations times on nums, where in each operation you:
    Select an index i that was not selected in any previous operations.
  Add an integer in the range [-k, k] to nums[i].
  
  Return the maximum possible frequency of any element in nums after performing the operations.
  
  Example 1:
  Input: nums = [1,4,5], k = 1, numOperations = 2
  Output: 2
  Explanation:
  We can achieve a maximum frequency of two by:
  Adding 0 to nums[1], after which nums becomes [1, 4, 5].
  Adding -1 to nums[2], after which nums becomes [1, 4, 4].
  
  Example 2:
  Input: nums = [5,11,20,20], k = 5, numOperations = 1
  Output: 2
  Explanation:
  We can achieve a maximum frequency of two by:
  Adding 0 to nums[1].
*/
use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort();

        let mut l: usize = 0;
        let mut r: usize = l;
        let mut s = 0;
        let mut l2: usize = 0;
        let mut s2 = 0;
        let mut prev = -1;
        let mut cnt = 0;

        let mut res = 0;
        let n = nums.len();

        nums.iter().for_each(|&num| {
            if num == prev {
                cnt += 1;
            } else {
                prev = num;
                cnt = 1;
            }

            while nums[l] < num - k {
                s -= 1;
                l += 1;
            }
            while r < n && nums[r] <= num + k {
                s += 1;
                r += 1;
            }

            res = res.max(cnt + num_operations.min(s - cnt));

            s2 += 1;
            while nums[l2] < num - 2 * k {
                s2 -= 1;
                l2 += 1;
            }

            res = res.max(num_operations.min(s2));
        });

        res
    }
}
