/*
  Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.
  A subarray is a contiguous part of an array.

  Example 1:
    Input: nums = [4,5,0,-2,-3,1], k = 5
    Output: 7
    Explanation: There are 7 subarrays with a sum divisible by k = 5:
    [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
    
  Example 2:
    Input: nums = [5], k = 9
    Output: 0
*/
public class Solution {
    public int SubarraysDivByK(int[] nums, int k) {
        int result = 0, sum = 0;
        Span<int> map = stackalloc int[k];
        ++map[0];

        foreach (int num in nums) {
            sum += num;
            int mod = sum % k;

            if (mod < 0) mod += k;

            result += map[mod];
            ++map[mod];
        }

        return result;
    }
}
