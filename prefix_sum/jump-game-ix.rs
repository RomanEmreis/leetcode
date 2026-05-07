/*
  3660. Jump Game IX
  
  You are given an integer array nums.
  From any index i, you can jump to another index j under the following rules:
      Jump to index j where j > i is allowed only if nums[j] < nums[i].
      Jump to index j where j < i is allowed only if nums[j] > nums[i].
  
  For each index i, find the maximum value in nums that can be reached by following any sequence of valid jumps starting at i.
  
  Return an array ans where ans[i] is the maximum value reachable starting from index i.
  
  Example 1:
  Input: nums = [2,1,3]
  Output: [2,2,3]
  Explanation:
      For i = 0: No jump increases the value.
      For i = 1: Jump to j = 0 as nums[j] = 2 is greater than nums[i].
      For i = 2: Since nums[2] = 3 is the maximum value in nums, no jump increases the value.
  Thus, ans = [2, 2, 3].
  
  Example 2:
  Input: nums = [2,3,1]
  Output: [3,3,3]
  Explanation:
      For i = 0: Jump forward to j = 2 as nums[j] = 1 is less than nums[i] = 2, then from i = 2 jump to j = 1 as nums[j] = 3 is greater than nums[2].
      For i = 1: Since nums[1] = 3 is the maximum value in nums, no jump increases the value.
      For i = 2: Jump to j = 1 as nums[j] = 3 is greater than nums[2] = 1.
  Thus, ans = [3, 3, 3].
*/
impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0_i32; n];

        res[0] = nums[0];
        for i in 1..n {
            res[i] = res[i - 1].max(nums[i]);
        }

        let mut suf_min = i32::MAX;
        for i in (0..n - 1).rev() {
            if res[i] > suf_min {
                res[i] = res[i + 1];
            }
            suf_min = suf_min.min(nums[i + 1]);
        }

        res
    }
}
