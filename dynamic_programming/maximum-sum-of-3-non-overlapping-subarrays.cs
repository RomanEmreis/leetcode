/*
  689. Maximum Sum of 3 Non-Overlapping Subarrays
  
  Given an integer array nums and an integer k, find three non-overlapping subarrays of length k with maximum sum and return them.
  
  Return the result as a list of indices representing the starting position of each interval (0-indexed). If there are multiple answers, return the lexicographically smallest one.
  
  Example 1:
  Input: nums = [1,2,1,2,6,7,5,1], k = 2
  Output: [0,3,5]
  Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
  We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.
  
  Example 2:
  Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
  Output: [0,2,4]
*/
public class Solution {
    public int[] MaxSumOfThreeSubarrays(int[] nums, int k) {
        int[] result = new int[3];

        Span<int> s = stackalloc int[4];
        Span<int> max = stackalloc int[2];
        Span<int> idx = stackalloc int[3];

        for (int i = k * 2; i < nums.Length; ++i) {
            s[1] += nums[i - k * 2];
            s[2] += nums[i - k];
            s[3] += nums[i];

            if (i >= k * 3 - 1) {
                if (s[1] > max[0]) {
                    max[0] = s[1];
                    idx[0] = i - k * 3 + 1;
                }
                if (max[0] + s[2] > max[1]) {
                    max[1] = s[2] + max[0];
                    idx[1] = idx[0];
                    idx[2] = i - k * 2 + 1;
                }
                if (max[1] + s[3] > s[0]) {
                    s[0] = s[3] + max[1];
                    result[0] = idx[1];
                    result[1] = idx[2];
                    result[2] = i - k + 1;
                }
                s[1] -= nums[i - k * 3 + 1];
                s[2] -= nums[i - k * 2 + 1];
                s[3] -= nums[i - k + 1];
            }
        }

        return result;
    }
}
