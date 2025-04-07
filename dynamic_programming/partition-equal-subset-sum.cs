/*
  416. Partition Equal Subset Sum
  
  Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
  
  Example 1:
  Input: nums = [1,5,11,5]
  Output: true
  Explanation: The array can be partitioned as [1, 5, 5] and [11].
  
  Example 2:
  Input: nums = [1,2,3,5]
  Output: false
  Explanation: The array cannot be partitioned into equal sum subsets.
*/
public class Solution {
    public bool CanPartition(int[] nums) {
        int sum = 0;
        foreach (int n in nums) sum += n;

        if (sum % 2  == 1) return false;

        sum /= 2;

        Span<bool> dp = stackalloc bool[sum + 1];
        dp[0] = true;

        foreach (int n in nums) {
            for (int i = sum; i >= n; --i) {
                dp[i] = dp[i] || dp[i - n];
            }
        }

        return dp[sum];
    }
}
