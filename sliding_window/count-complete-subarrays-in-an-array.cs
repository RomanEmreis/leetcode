/*
  2799. Count Complete Subarrays in an Array
  
  You are given an array nums consisting of positive integers.
  
  We call a subarray of an array complete if the following condition is satisfied:
  
  The number of distinct elements in the subarray is equal to the number of distinct elements in the whole array.
  Return the number of complete subarrays.
  
  A subarray is a contiguous non-empty part of an array.
  
  Example 1:
  Input: nums = [1,3,1,2,2]
  Output: 4
  Explanation: The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
  
  Example 2:
  Input: nums = [5,5,5,5]
  Output: 10
  Explanation: The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
*/
public class Solution {
    public int CountCompleteSubarrays(int[] nums) {
        Span<int> ints = stackalloc int[2001];
        int count = 0;
        foreach (int n in nums) {
            if (ints[n] == 0) ++count;
            ++ints[n];
        } 

        int result = 0;
        for (int i = 0; i <= nums.Length - count; ++i) {
            ints = stackalloc int[2001];
            int curr = 0;
            for (int j = i; j < nums.Length; ++j) {
                if (ints[nums[j]] == 0) ++curr;
                ++ints[nums[j]];

                if (curr == count) ++result;
            }
        }        

        return result;
    }
}
