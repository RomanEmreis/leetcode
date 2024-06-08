/*
  Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
  A good subarray is a subarray where:
    its length is at least two, and
    the sum of the elements of the subarray is a multiple of k.

  Note that:
    A subarray is a contiguous part of the array.
    An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k. 

  Example 1:
    Input: nums = [23,2,4,6,7], k = 6
    Output: true
    Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
*/
public class Solution {
    public bool CheckSubarraySum(int[] nums, int k) {
        Dictionary<int, int> map = [];
        map.Add(0, -1);

        int sum = 0;
        for (int i = 0; i < nums.Length; ++i) {
            sum += nums[i];
            int mod = sum % k;
            if (!map.TryGetValue(mod, out int j)) map.Add(mod, i);
            else if (i - j >= 2) return true;
        }

        return false;
    }
}
