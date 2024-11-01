/*
  A fancy string is a string where no three consecutive characters are equal.
  
  Given a string s, delete the minimum possible number of characters from s to make it fancy.
  
  Return the final string after the deletion. It can be shown that the answer will always be unique.
  
  Example 1:
    Input: s = "leeetcode"
    Output: "leetcode"
    Explanation:
    Remove an 'e' from the first group of 'e's to create "leetcode".
    No three consecutive characters are equal, so return "leetcode".
*/
public class Solution {
    public string MakeFancyString(string s) {
        StringBuilder sb = new();
        sb.Append(s[0]);

        for (int i = 1; i < s.Length; ++i) {
            if (sb[^1] != s[i] || sb.Length < 2 || sb[^2] != s[i]) sb.Append(s[i]);
        }

        return sb.ToString();
    }
}
