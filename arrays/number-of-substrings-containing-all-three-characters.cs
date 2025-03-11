/*
  1358. Number of Substrings Containing All Three Characters
  
  Given a string s consisting only of characters a, b and c.
  
  Return the number of substrings containing at least one occurrence of all these characters a, b and c.
  
  Example 1:
  Input: s = "abcabc"
  Output: 10
  Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again). 
  
  Example 2:
  Input: s = "aaacb"
  Output: 3
  Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb". 
  
  Example 3:
  Input: s = "abc"
  Output: 1
*/
public class Solution {
    public int NumberOfSubstrings(string s) {
        int result = 0;
        Span<int> dp = [-1, -1, -1];
        for (int i = 0; i < s.Length; ++i) {
            dp[s[i] - 'a'] = i;
            result += Math.Min(dp[0], Math.Min(dp[1], dp[2])) + 1;
        }
        return result;
    }
}
