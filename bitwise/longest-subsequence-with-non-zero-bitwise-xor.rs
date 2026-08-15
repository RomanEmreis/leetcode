/*
  3702. Longest Subsequence With Non-Zero Bitwise XOR
  
  You are given an integer array nums.
  
  Return the length of the longest in nums whose bitwise XOR is non-zero. If no such subsequence exists, return 0.
  
  Example 1:
  Input: nums = [1,2,3]
  Output: 2
  Explanation:
  One longest subsequence is [2, 3]. The bitwise XOR is computed as 2 XOR 3 = 1, which is non-zero.
  
  Example 2:
  Input: nums = [2,3,4]
  Output: 3
  Explanation:
  The longest subsequence is [2, 3, 4]. The bitwise XOR is computed as 2 XOR 3 XOR 4 = 5, which is non-zero.
*/
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let (xor, has_non_zero) = nums.into_iter()
            .fold((0, 0), |(xor, has_non_zero), num| (
                xor ^ num,
                has_non_zero | num
            ));

        if xor != 0 {
            n
        } else if has_non_zero != 0 {
            n - 1
        } else {
            0
        }
    }
}
