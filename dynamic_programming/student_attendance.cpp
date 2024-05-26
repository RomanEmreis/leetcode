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
class Solution {
private:
    static const int MOD = 1000000007;

public:
    int checkRecord(int n) {
        if (n == 0) return 1;
        if (n == 1) return 3;

        vector<int> f(n+1), g(n+1);
        f[0] = g[0] = 1;
        f[1] = 3;
        f[2] = 8;
        g[1] = 2;
        g[2] = 4;
        for (int i = 3; i <= n; ++i) {
            g[i] = ((g[i-1] + g[i-2]) % MOD + g[i-3]) % MOD;
            f[i] = (((f[i-1] + f[i-2]) % MOD + f[i-3]) % MOD + g[i]) % MOD;
        }
        return f[n];
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
