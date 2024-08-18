/*
  An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
  Given an integer n, return the nth ugly number.

  Example 1:
    Input: n = 10
    Output: 12
    Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
  
  Example 2:
    Input: n = 1
    Output: 1
    Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
*/
public class Solution {
    public int NthUglyNumber(int n) {
        if (n <= 0) return 0;
        if (n == 1) return 1;

        Span<int> dp = stackalloc int[n];
        dp[0] = 1;

        int i = 1, i2 = 0, i3 = 0, i5 = 0;
        int f2 = 2, f3 = 3, f5 = 5;
        while (i < n) {
            int min = Math.Min(f2, Math.Min(f3, f5));
            dp[i++] = min;

            if (min == f2) f2 = 2 * dp[++i2];
            else if (min == f3) f3 = 3 * dp[++i3];
            else if (min == f5) f5 = 5 * dp[++i5];
        }

        return dp[^1];
    }
}
