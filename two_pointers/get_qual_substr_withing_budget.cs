/*
  You are given two strings s and t of the same length and an integer maxCost.
  You want to change s to t. Changing the ith character of s to ith character of t costs |s[i] - t[i]| (i.e., the absolute difference between the ASCII values of the characters).

  Return the maximum length of a substring of s that can be changed to be the same as the corresponding substring of t with a cost less than or equal to maxCost. If there is no substring from s that can be changed to its corresponding substring from t, return 0.

  Example:
    Input: s = "abcd", t = "bcdf", maxCost = 3
    Output: 3
    Explanation: "abc" of s can change to "bcd".
    That costs 3, so the maximum length is 3.
*/
public class Solution {
    public int EqualSubstring(string s, string t, int maxCost) {
        int result = 0;
        int cost = 0;
        int start = 0;

        for (int end = 0; end < s.Length; ++end) {
            cost += Math.Abs(s[end] - t[end]);
            while(cost > maxCost) {
                cost -= Math.Abs(s[start] - t[start]);
                ++start;   
            }

            result = Math.Max(result, end - start + 1);
        }

        return result;
    }
}
