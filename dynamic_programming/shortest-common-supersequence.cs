/*
  1092. Shortest Common Supersequence 
  
  Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences. If there are multiple valid strings, return any of them.
  
  A string s is a subsequence of string t if deleting some number of characters from t (possibly 0) results in the string s.
  
  Example 1:
  Input: str1 = "abac", str2 = "cab"
  Output: "cabac"
  Explanation: 
  str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
  str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
  The answer provided is the shortest such string that satisfies these properties.
  
  Example 2:
  Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
  Output: "aaaaaaaa"
   
  Constraints:
  1 <= str1.length, str2.length <= 1000
  str1 and str2 consist of lowercase English letters.
*/
public class Solution {
    public string ShortestCommonSupersequence(string str1, string str2) {
        int m = str1.Length;
        int n = str2.Length;

        int[,] dp = new int[m + 1, n + 1];
        for (int i = 1; i <= m; ++i) {
            for (int j = 1; j <= n; ++j) {
                if (str1[i - 1] == str2[j - 1]) dp[i, j] = dp[i - 1, j - 1] + 1;
                else dp[i, j] = Math.Max(dp[i - 1, j], dp[i, j - 1]);
            }
        }

        StringBuilder sb = new();
        while (m > 0 || n > 0) {
            if (m == 0) sb.Append(str2[--n]);
            else if (n == 0) sb.Append(str1[--m]);
            else {
                if (dp[m, n] == dp[m - 1, n]) sb.Append(str1[--m]);
                else if (dp[m, n] == dp[m, n - 1]) sb.Append(str2[--n]);
                else {
                    sb.Append(str1[--m]);
                    --n;
                }
            }
        }

        for (int i = 0, j = sb.Length - 1; i < j; ++i, --j) {
            (sb[i], sb[j]) = (sb[j], sb[i]);
        }

        return sb.ToString();
    }
}
