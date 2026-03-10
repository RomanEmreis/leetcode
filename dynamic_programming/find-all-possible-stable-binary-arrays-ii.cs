/*
  3130. Find All Possible Stable Binary Arrays II
  
  You are given 3 positive integers zero, one, and limit.
  A arr is called stable if:
      The number of occurrences of 0 in arr is exactly zero.
      The number of occurrences of 1 in arr is exactly one.
      Each of arr with a size greater than limit must contain both 0 and 1.
  
  Return the total number of stable binary arrays.
  
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: zero = 1, one = 1, limit = 2
  Output: 2
  Explanation:
  The two possible stable binary arrays are [1,0] and [0,1].
  
  Example 2:
  Input: zero = 1, one = 2, limit = 1
  Output: 1
  Explanation:
  The only possible stable binary array is [1,0,1].
  
  Example 3:
  Input: zero = 3, one = 3, limit = 2
  Output: 14
  Explanation:
  All the possible stable binary arrays are [0,0,1,0,1,1], [0,0,1,1,0,1], [0,1,0,0,1,1], [0,1,0,1,0,1], [0,1,0,1,1,0],
    [0,1,1,0,0,1], [0,1,1,0,1,0], [1,0,0,1,0,1], [1,0,0,1,1,0], [1,0,1,0,0,1], [1,0,1,0,1,0], [1,0,1,1,0,0], [1,1,0,0,1,0], and [1,1,0,1,0,0].
*/
public class Solution {
    public int NumberOfStableArrays(int zero, int one, int limit) {
        const int Mod = 1_000_000_007;

        long[,,] dp = new long[zero + 1, one + 1, 2];

        for (int i = 0; i <= Math.Min(zero, limit); ++i)
            dp[i, 0, 0] = 1;

        for (int j = 0; j <= Math.Min(one, limit); ++j)
            dp[0, j, 1] = 1;

        for (int i = 1; i <= zero; ++i)
            for (int j = 1; j <= one; ++j) {
                dp[i, j, 0] = (
                    dp[i - 1, j, 0] + dp[i - 1, j, 1] - (i - limit < 1 ? 0 : dp[i - limit - 1, j, 1]) + Mod
                ) % Mod;

                dp[i, j, 1] = (
                    dp[i, j - 1, 0] + dp[i, j - 1, 1] - (j - limit < 1 ? 0 : dp[i, j - limit - 1, 0]) + Mod
                ) % Mod;
            }

        return (int) ((dp[zero, one, 0] + dp[zero, one, 1]) % Mod);
    }
}
