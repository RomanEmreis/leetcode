/*
  You are given a 0-indexed string s and a dictionary of words dictionary. You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. 
  There may be some extra characters in s which are not present in any of the substrings.
  
  Return the minimum number of extra characters left over if you break up s optimally.
  
  Example 1:
    Input: s = "leetscode", dictionary = ["leet","code","leetcode"]
    Output: 1
    Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8. There is only 1 unused character (at index 4), so we return 1.
  
  Example 2:
    Input: s = "sayhelloworld", dictionary = ["hello","world"]
    Output: 3
    Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12. The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. Hence, we return 3.
*/
public class Solution {
    public int MinExtraChar(string s, string[] dictionary) {
        int n = s.Length;
        Span<int> dp = stackalloc int[n + 1];
        HashSet<string> dict = new(dictionary);

        for (int i = 0; i <= n; ++i) {
            dp[i] = i;
        }

        for (int i = 1; i <= n; ++i) {
            dp[i] = dp[i - 1] + 1;
            for (int j = 0; j < i; ++j) {
                string sub = s[j..i];
                if (dict.Contains(sub)) {
                    dp[i] = Math.Min(dp[i], dp[j]);
                }
            }
        }

        return dp[n];
    }
}
