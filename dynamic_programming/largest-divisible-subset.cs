/*
  368. Largest Divisible Subset
  
  Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
      answer[i] % answer[j] == 0, or
      answer[j] % answer[i] == 0
  
  If there are multiple solutions, return any of them.
  
  Example 1:
  Input: nums = [1,2,3]
  Output: [1,2]
  Explanation: [1,3] is also accepted.
  
  Example 2:
  Input: nums = [1,2,4,8]
  Output: [1,2,4,8]
*/
public class Solution {
    public IList<int> LargestDivisibleSubset(int[] nums) {
        Array.Sort(nums);
        Span<int> dp = stackalloc int[nums.Length];
        int k = 0;

        for (int i = 0; i < nums.Length; ++i) {
            dp[i] = 1;
            for (int j = 0; j < i; ++j) {
                if (nums[i] % nums[j] == 0) {
                    dp[i] = Math.Max(dp[i], dp[j] + 1);
                }
            }
            if (dp[k] < dp[i]) k = i;
        }

        int m = dp[k];
        List<int> result = [];
        for (int i = k; m > 0; --i) {
            if (nums[k] % nums[i] == 0 && dp[i] == m) {
                result.Add(nums[i]);
                k = i;
                --m;
            }
        }
        return result;
    }
}
