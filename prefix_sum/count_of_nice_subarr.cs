/*
  Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.
  Return the number of nice sub-arrays.

  Example:
    Input: nums = [1,1,2,1,1], k = 3
    Output: 2
    Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
*/
public class Solution {
    public int NumberOfSubarrays(int[] nums, int k) {
        int result = 0, sum = 0;
        Span<int> map = stackalloc int[nums.Length + 1];
        ++map[0];

        foreach (int num in nums) {
            sum += num & 1;
            if (sum - k >= 0) result += map[sum - k];
            ++map[sum];
        }

        return result;
    }
}
