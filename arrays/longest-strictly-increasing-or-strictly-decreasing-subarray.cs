/*
  3105. Longest Strictly Increasing or Strictly Decreasing Subarray
  
  Example 1:
  Input: nums = [1,4,3,3,2]
  Output: 2
  Explanation:
  The strictly increasing subarrays of nums are [1], [2], [3], [3], [4], and [1,4].
  The strictly decreasing subarrays of nums are [1], [2], [3], [3], [4], [3,2], and [4,3].
  Hence, we return 2.
  
  Example 2:
  Input: nums = [3,3,3,3]
  Output: 1
  Explanation:
  The strictly increasing subarrays of nums are [3], [3], [3], and [3].
  The strictly decreasing subarrays of nums are [3], [3], [3], and [3].
  Hence, we return 1.
  
  Example 3:
  Input: nums = [3,2,1]
  Output: 3
  Explanation:
  The strictly increasing subarrays of nums are [3], [2], and [1].
  The strictly decreasing subarrays of nums are [3], [2], [1], [3,2], [2,1], and [3,2,1].
  Hence, we return 3.
*/
public class Solution {
    public int LongestMonotonicSubarray(int[] nums) {
        int result = 1;
        for (int i = 1, len = 1; i < nums.Length; ++i) {
            if (nums[i - 1] < nums[i]) result = Math.Max(result, ++len);
            else len = 1;
        }
        for (int i = 1, len = 1; i < nums.Length; ++i) {
            if (nums[i - 1] > nums[i]) result = Math.Max(result, ++len);
            else len = 1;
        }
        return result;
    }
}
