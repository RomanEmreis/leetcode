/*
  An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:
    'A': Absent.
    'L': Late.
    'P': Present.
  Any student is eligible for an attendance award if they meet both of the following criteria:

  The student was absent ('A') for strictly fewer than 2 days total.
  The student was never late ('L') for 3 or more consecutive days.
  Given an integer n, return the number of possible attendance records of length n that make a student eligible for an attendance award. The answer may be very large, so return it modulo 109 + 7.

  Example:
    Input: n = 2
    Output: 8
    Explanation: There are 8 records with length 2 that are eligible for an award:
    "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
    Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
*/
public class Solution {
    private const int MOD = 1000000007;
    public int CheckRecord(int n) {
        if (n == 0) return 1;
        if (n == 1) return 3;

        Span<long> dp = [1,1,0,1,0,0];
        for (int i = 1; i < n; ++i) {
            Span<long> next = [
                dp[0] + dp[1] + dp[2],
                dp[0],
                dp[1],
                dp[0] + dp[1] +dp[2] + dp[3] + dp[4] + dp[5],
                dp[3],
                dp[4]
            ];

            for (int j = 0; j < dp.Length; ++j) {
                dp[j] = next[j] % MOD;
            }
        }

        return (int)((dp[0] + dp[1] +dp[2] + dp[3] + dp[4] + dp[5]) % MOD);
    }
}
