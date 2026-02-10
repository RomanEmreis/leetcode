/*
  3719. Longest Balanced Subarray I
  
  You are given an integer array nums.
  A is called balanced if the number of distinct even numbers in the subarray is equal to the number of distinct odd numbers.
  
  Return the length of the longest balanced subarray.
  
  Example 1:
  Input: nums = [2,5,4,3]
  Output: 4
  Explanation:
      The longest balanced subarray is [2, 5, 4, 3].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [5, 3]. Thus, the answer is 4.
  
  Example 2:
  Input: nums = [3,2,2,5,4]
  Output: 5
  Explanation:
      The longest balanced subarray is [3, 2, 2, 5, 4].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [3, 5]. Thus, the answer is 5.
  
  Example 3:
  Input: nums = [1,2,3,2]
  Output: 3
  Explanation:
      The longest balanced subarray is [2, 3, 2].
      It has 1 distinct even number [2] and 1 distinct odd number [3]. Thus, the answer is 3.
*/
use std::collections::HashSet;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let mut set = HashSet::new();
            let mut cnt = [0; 2];
            for j in i..n {
                if !set.contains(&nums[j]) {
                    set.insert(nums[j]);
                    cnt[(nums[j] & 1) as usize] += 1;
                }
                if cnt[0] == cnt[1] {
                    res = res.max((j - i + 1) as i32);
                }
            }
        }
        res        
    }
}
