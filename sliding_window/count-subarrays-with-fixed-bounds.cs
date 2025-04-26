/*
  2444. Count Subarrays With Fixed Bounds
  
  You are given an integer array nums and two integers minK and maxK.
  A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
      The minimum value in the subarray is equal to minK.
      The maximum value in the subarray is equal to maxK.
  
  Return the number of fixed-bound subarrays.
  A subarray is a contiguous part of an array.
  
  Example 1:
  Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
  Output: 2
  Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
  
  Example 2:
  Input: nums = [1,1,1,1], minK = 1, maxK = 1
  Output: 10
  Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
*/
public class Solution {
    public long CountSubarrays(int[] nums, int minK, int maxK) {
       long result = 0;
       int k = -1;
       int min = -1;
       int max = -1;
       for (int i = 0; i < nums.Length; ++i) {
            if (nums[i] < minK || nums[i] > maxK) {
                k = i;
            }
            if (nums[i] == minK) min = i;
            if (nums[i] == maxK) max = i;

            result += Math.Max(0, Math.Min(max, min) - k);
       }
       return result; 
    }
}
