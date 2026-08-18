/*
  3471. Find the Largest Almost Missing Integer
  
  You are given an integer array nums and an integer k.
  An integer x is almost missing from nums if x appears in exactly one subarray of size k within nums.
  
  Return the largest almost missing integer from nums. If no such integer exists, return -1.
  A subarray is a contiguous sequence of elements within an array.
  
  Example 1:
  Input: nums = [3,9,2,1,7], k = 3
  Output: 7
  Explanation:
      1 appears in 2 subarrays of size 3: [9, 2, 1] and [2, 1, 7].
      2 appears in 3 subarrays of size 3: [3, 9, 2], [9, 2, 1], [2, 1, 7].
      3 appears in 1 subarray of size 3: [3, 9, 2].
      7 appears in 1 subarray of size 3: [2, 1, 7].
      9 appears in 2 subarrays of size 3: [3, 9, 2], and [9, 2, 1].
  
  We return 7 since it is the largest integer that appears in exactly one subarray of size k.
  
  Example 2:
  Input: nums = [3,9,7,2,1,7], k = 4
  Output: 3
  Explanation:
      1 appears in 2 subarrays of size 4: [9, 7, 2, 1], [7, 2, 1, 7].
      2 appears in 3 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1], [7, 2, 1, 7].
      3 appears in 1 subarray of size 4: [3, 9, 7, 2].
      7 appears in 3 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1], [7, 2, 1, 7].
      9 appears in 2 subarrays of size 4: [3, 9, 7, 2], [9, 7, 2, 1].
  
  We return 3 since it is the largest and only integer that appears in exactly one subarray of size k.
  
  Example 3:
  Input: nums = [0,0], k = 1
  Output: -1
  Explanation:
  There is no integer that appears in only one subarray of size 1.
*/
impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        let mut seen = 0u64;
        let mut repeated = 0u64;

        for &value in &nums {
            let bit = 1u64 << value as u32;

            repeated |= seen & bit;
            seen |= bit;
        }

        let eligible = if k == n {
            seen
        } else {
            let unique = seen & !repeated;

            if k == 1 {
                unique
            } else {
                let endpoints = (1u64 << nums[0] as u32) | (1u64 << nums[n - 1] as u32);
                unique & endpoints
            }
        };

        if eligible == 0 {
            -1
        } else {
            (63 - eligible.leading_zeros()) as i32
        }
    }
}
