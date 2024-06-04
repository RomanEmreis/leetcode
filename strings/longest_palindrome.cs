/*
  Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
  Letters are case sensitive, for example, "Aa" is not considered a palindrome.

  Example:
    Input: s = "abccccdd"
    Output: 7
    Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
*/
public class Solution {
    public int LongestPalindrome(string s) {
        if (s.Length == 1) return 1;

        int result = 0;
        HashSet<char> set = [];

        foreach (char c in s) {
            if (!set.Add(c)) {
                set.Remove(c);
                result += 2;
            }
        }

        if (set.Any()) ++result;

        return result;
    }
}
