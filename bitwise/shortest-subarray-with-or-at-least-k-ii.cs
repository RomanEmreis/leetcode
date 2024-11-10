/*
  You are given an array nums of non-negative integers and an integer k.
  An array is called special if the bitwise OR of all of its elements is at least k.
  
  Return the length of the shortest special non-empty subarray of nums, or return -1 if no special subarray exists.
  
  Example 1:
    Input: nums = [1,2,3], k = 2
    Output: 1
    Explanation:
    The subarray [3] has OR value of 3. Hence, we return 1.
  
  Example 2:
    Input: nums = [2,1,8], k = 10
    Output: 3
    Explanation:
    The subarray [2,1,8] has OR value of 11. Hence, we return 3.
*/
public class Solution {
    public int MinimumSubarrayLength(int[] nums, int k) {
        int n = nums.Length;
        Span<int> count = stackalloc int[32];
        int result = n + 1;

        for (int i = 0, j = 0, s = 0; j < n; ++j) {
            s |= nums[j];
            for (int b = 0; b < 32; ++b) {
                if ((nums[j] >> b & 1) == 1) ++count[b];
            }

            while (s >= k && i <= j) {
                result = Math.Min(result, j - i + 1);
                for (int b = 0; b < 32; ++b) {
                    if ((nums[i] >> b & 1) == 1) {
                        if (--count[b] == 0) s ^= 1 << b;
                    }
                }
                ++i;
            }
        }

        return result > n ? -1 : result;
    }
}
