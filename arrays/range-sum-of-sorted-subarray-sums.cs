/*
  You are given the array nums consisting of n positive integers. You computed the sum of all non-empty continuous subarrays from the array and then sorted them in non-decreasing order, creating a new array of n * (n + 1) / 2 numbers.
  Return the sum of the numbers from index left to index right (indexed from 1), inclusive, in the new array. Since the answer can be a huge number return it modulo 109 + 7.

  Example:
    Input: nums = [1,2,3,4], n = 4, left = 1, right = 5
    Output: 13 
    Explanation: All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13. 
*/
public class Solution {
    public int RangeSum(int[] nums, int n, int left, int right) {
        const int mod = 1000000007;
        List<int> subArraySums = [];

        for (int i = 0; i < nums.Length; ++i) {
            int sum = nums[i];
            subArraySums.Add(sum);
            for (int j = i + 1; j < nums.Length; ++j) {
                sum += nums[j];
                subArraySums.Add(sum);
            }
        }

        subArraySums.Sort();

        long result = 0;
        for (int i = left - 1; i < right; ++i) {
            result += subArraySums[i];
        }        

        return (int) (result % mod);
    }
}
