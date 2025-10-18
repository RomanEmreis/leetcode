/*
  3397. Maximum Number of Distinct Elements After Operations
  
  You are given an integer array nums and an integer k.
  You are allowed to perform the following operation on each element of the array at most once:
      Add an integer in the range [-k, k] to the element.
  
  Return the maximum possible number of distinct elements in nums after performing the operations.
  
  Example 1:
  Input: nums = [1,2,2,3,3,4], k = 2
  Output: 6
  Explanation:
  nums changes to [-1, 0, 1, 2, 3, 4] after performing operations on the first four elements.
  
  Example 2:
  Input: nums = [4,4,4,4], k = 1
  Output: 3
  Explanation:
  By adding -1 to nums[0] and 1 to nums[1], nums changes to [3, 5, 4, 4].
*/
impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let mut res = 0;
        let mut used = i32::MIN;

        nums.iter().for_each(|num| {
            if used < num + k {
                used = std::cmp::max(used + 1, num - k);
                res += 1;
            }
        });

        res
    }
}
