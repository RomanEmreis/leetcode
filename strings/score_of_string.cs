/*
  You are given a string s. The score of a string is defined as the sum of the absolute difference between the ASCII values of adjacent characters.
  Return the score of s.

  Example:
    Input: s = "hello"
    Output: 13
    Explanation:
    The ASCII values of the characters in s are: 'h' = 104, 'e' = 101, 'l' = 108, 'o' = 111. So, the score of s would be |104 - 101| + |101 - 108| + |108 - 108| + |108 - 111| = 3 + 7 + 0 + 3 = 13.
*/
public class Solution {
    public int ScoreOfString(string s) {
        int result = 0;

        for (int i = 1; i < s.Length; ++i) {
            result += Math.Abs((int)s[i - 1] - (int)s[i]);
        }

        return result;
    }
}
