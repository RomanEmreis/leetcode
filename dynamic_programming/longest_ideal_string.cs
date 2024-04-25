public class Solution {
    public int LongestIdealString(string s, int k) {
        const char a = 'a';
        const int maxChars = 25;

        Span<int> dp = stackalloc int[maxChars + 1];
        int max = 1;

        for (int i = 0; i < s.Length; ++i) {
            int temp = 0;
            for (int j = Math.Max(0, s[i] - a - k); j <= Math.Min(maxChars, s[i] - a + k); j++) {
                temp = Math.Max(temp, dp[j]);
            }

            dp[s[i] - a] = temp + 1;
            max = Math.Max(max, temp + 1);
        }

        return max;
    }
}
