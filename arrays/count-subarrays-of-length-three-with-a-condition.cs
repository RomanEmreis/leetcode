/*
  3392. Count Subarrays of Length Three With a Condition
  
  Given an integer array nums, return the number of of length 3 such that the sum of the first and third numbers equals exactly half of the second number.
  
  Example 1:
  Input: nums = [1,2,1,4,1]
  Output: 1
  Explanation:
  Only the subarray [1,4,1] contains exactly 3 elements where the sum of the first and third numbers equals half the middle number.
  
  Example 2:
  Input: nums = [1,1,1]
  Output: 0
  Explanation:
  [1,1,1] is the only subarray of length 3. However, its first and third numbers do not add to half the middle number.
*/
public class Solution {
    public int CountSubarrays(int[] nums) {
        int result = 0;
        for (int i = 2; i < nums.Length; ++i) {
            double sum = 1D * nums[i] + nums[i - 2];
            double half = nums[i - 1] / 2.0;
            if (half == sum) {
                ++result;
            }
        }
        return result;
    }
}
