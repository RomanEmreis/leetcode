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
const MOD = 1000000007;
func checkRecord(n int) int {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 3;
    }

    dp := []int64 { 1, 1, 0, 1, 0, 0 };
    for i := 1; i < n; i++ {
        next := make([]int64, 6);
        next[0] = dp[0] + dp[1] + dp[2];
        next[1] = dp[0];
        next[2] = dp[1];
        next[3] = dp[0] + dp[1] +dp[2] + dp[3] + dp[4] + dp[5];
        next[4] = dp[3];
        next[5] = dp[4];

        for j := 0; j < 6; j++ {
            dp[j] = next[j] % MOD;
        }
    }

    return int((dp[0] + dp[1] +dp[2] + dp[3] + dp[4] + dp[5]) % MOD);
}
