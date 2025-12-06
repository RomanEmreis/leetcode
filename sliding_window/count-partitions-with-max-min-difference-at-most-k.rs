/*
  3578. Count Partitions With Max-Min Difference at Most K
  
  You are given an integer array nums and an integer k. Your task is to partition nums into one or more non-empty contiguous segments such that in each segment, 
  the difference between its maximum and minimum elements is at most k.
  
  Return the total number of ways to partition nums under this condition.
  
  Since the answer may be too large, return it modulo 109 + 7.
  
  Example 1:
  Input: nums = [9,4,1,3,7], k = 4
  Output: 6
  Explanation:
  There are 6 valid partitions where the difference between the maximum and minimum elements in each segment is at most k = 4:
      [[9], [4], [1], [3], [7]]
      [[9], [4], [1], [3, 7]]
      [[9], [4], [1, 3], [7]]
      [[9], [4, 1], [3], [7]]
      [[9], [4, 1], [3, 7]]
      [[9], [4, 1, 3], [7]]
  
  Example 2:
  Input: nums = [3,3,4], k = 0
  Output: 2
  Explanation:
  There are 2 valid partitions that satisfy the given conditions:
      [[3], [3], [4]]
      [[3, 3], [4]]
*/
use std::collections::VecDeque;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        let mut dp: Vec<i64> = vec![0; n + 1];
        let mut prefix: Vec<i64> = vec![0; n + 1];
        let mut min: VecDeque<usize> = VecDeque::new();
        let mut max: VecDeque<usize> = VecDeque::new();

        dp[0] = 1;
        prefix[0] = 1;

        let mut i = 0;
        let mut j = 0;

        while i < n {
            while let Some(last) = max.back() && nums[*last] <= nums[i] {
                max.pop_back();
            }
            while let Some(last) = min.back() && nums[*last] >= nums[i] {
                min.pop_back();
            }

            max.push_back(i);
            min.push_back(i);

            while let Some(&mx) = max.front() && let Some(&mn) = min.front() && nums[mx] - nums[mn] > k {
                if mx == j {
                    max.pop_front();
                }
                if mn == j {
                    min.pop_front();
                }
                j += 1;
            }

            let prev = if j > 0 { prefix[j - 1] } else { 0 };
            dp[i + 1] = (prefix[i] - prev + MOD) % MOD;
            prefix[i + 1] = (prefix[i] + dp[i + 1]) % MOD;

            i += 1;
        }

        dp[n] as i32
    }
}
