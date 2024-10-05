/*
  Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
  In other words, return true if one of s1's permutations is the substring of s2.
  
  Example 1:
    Input: s1 = "ab", s2 = "eidbaooo"
    Output: true
    Explanation: s2 contains one permutation of s1 ("ba").
  
  Example 2:
    Input: s1 = "ab", s2 = "eidboaoo"
    Output: false
*/
public class Solution {
    public bool CheckInclusion(string s1, string s2) {
        if (s1.Length > s2.Length) return false;

        Span<int> chars1 = stackalloc int[26];
        Span<int> chars2 = stackalloc int[26];

        foreach (char ch in s1) {
            ++chars1[ch - 'a'];
        }

        for (int i = 0; i < s2.Length; ++i) {
            ++chars2[s2[i] - 'a'];

            if (i >= s1.Length) {
                --chars2[s2[i - s1.Length] - 'a'];
            }

            if (CompareArrays(ref chars1, ref chars2)) return true;
        }

        return false;

        static bool CompareArrays(ref Span<int> chars1, ref Span<int> chars2) {
            for (int j = 0; j < 26; ++j) {
                if (chars1[j] != chars2[j]) return false;
            }
            return true;
        }
    }
}
