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
class Solution {
public:
    int nthUglyNumber(int n) {
        vector<int> dp(n);
        dp[0] = 1;

        size_t i = 1, i2 = 0, i3 = 0, i5 = 0;
        int f2 = 2, f3 = 3, f5 = 5;
        while (i < n) {
            int m = min(f2, min(f3, f5));
            dp[i++] = m;

            if (m == f2) f2 = 2 * dp[++i2];
            if (m == f3) f3 = 3 * dp[++i3];
            if (m == f5) f5 = 5 * dp[++i5];
        }
        return dp[n - 1];
    }
};
