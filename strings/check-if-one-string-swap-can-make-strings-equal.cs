/*
  1790. Check if One String Swap Can Make Strings Equal
  
  You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
  
  Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
  
  Example 1:
  Input: s1 = "bank", s2 = "kanb"
  Output: true
  Explanation: For example, swap the first character with the last character of s2 to make "bank".
  
  Example 2:
  Input: s1 = "attack", s2 = "defend"
  Output: false
  Explanation: It is impossible to make them equal with one string swap.
  
  Example 3:
  Input: s1 = "kelb", s2 = "kelb"
  Output: true
  Explanation: The two strings are already equal, so no string swap operation is required.
*/
public class Solution {
    public bool AreAlmostEqual(string s1, string s2) {
        Span<(char, char)> pos = stackalloc (char, char)[2];
        for (int i = 0, j = 0; i < s1.Length; ++i) {
            bool eq = s1[i] == s2[i];
            if (!eq) {
                if (j == 2) return false;
                pos[j++] = (s1[i], s2[i]);
            }
        }

        var (a, b) = pos[0];
        var (x, y) = pos[1];

        return a == y && x == b;
    }
}
