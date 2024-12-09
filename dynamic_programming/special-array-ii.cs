/*
  3152. Special Array II
  
  An array is considered special if every pair of its adjacent elements contains two numbers with different parity.
  You are given an array of integer nums and a 2D integer matrix queries, where for queries[i] = [fromi, toi] your task is to check that subarray nums[fromi..toi] is special or not.
  
  Return an array of booleans answer such that answer[i] is true if nums[fromi..toi] is special.
  
  Example 1:
  Input: nums = [3,4,1,2,6], queries = [[0,4]]
  Output: [false]
  Explanation:
  The subarray is [3,4,1,2,6]. 2 and 6 are both even.
  
  Example 2:
  Input: nums = [4,3,1,6], queries = [[0,2],[2,3]]
  Output: [false,true]
  Explanation:
  The subarray is [4,3,1]. 3 and 1 are both odd. So the answer to this query is false.
  The subarray is [1,6]. There is only one pair: (1,6) and it contains numbers with different parity. So the answer to this query is true.
*/
public class Solution {
    public bool[] IsArraySpecial(int[] nums, int[][] queries) {
        Span<int> dp = stackalloc int[nums.Length];
        dp.Fill(1);
        for (int i = 1; i < nums.Length; ++i) {
            if ((nums[i - 1] % 2) != (nums[i] % 2)) {
                dp[i] = dp[i - 1] + 1;
            }
        }

        bool[] result = new bool[queries.Length];
        for (int i = 0; i < queries.Length; ++i) {
            int start = queries[i][0];
            int end = queries[i][1];
            result[i] = dp[end] >= end - start + 1;
        }
        return result;
    }
}
