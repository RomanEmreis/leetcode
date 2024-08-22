/*
  There is a strange printer with the following two special properties:
    The printer can only print a sequence of the same character each time.
    At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.

  Given a string s, return the minimum number of turns the printer needed to print it.

  Example 1:
    Input: s = "aaabbb"
    Output: 2
    Explanation: Print "aaa" first and then print "bbb".
  
  Example 2:
    Input: s = "aba"
    Output: 2
    Explanation: Print "aaa" first and then print "b" from the second place of the string, which will cover the existing character 'a'.
*/
class Solution {
public:
    int strangePrinter(string s) {
        int n = s.length();
        vector<vector<int>> dp(n, vector<int>(n));

        for (int i = n - 1; i >= 0; --i) {
            dp[i][i] = 1;
            for (size_t j = i + 1; j < n; ++j) {
                dp[i][j] = dp[i][j - 1] + 1;
                for (size_t k = i; k < j; ++k) {
                    int total = dp[i][k] + dp[k + 1][j];
                    if (s[k] == s[j]) --total;
                    dp[i][j] = min(dp[i][j], total);
                }
            }
        }

        return dp[0][n - 1];
    }
};