/*
  1400. Construct K Palindrome Strings
  
  Given a string s and an integer k, return true if you can use all the characters in s to construct k palindrome strings or false otherwise.
  
  Example 1:
  Input: s = "annabelle", k = 2
  Output: true
  Explanation: You can construct two palindromes using all characters in s.
  Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"
  
  Example 2:
  Input: s = "leetcode", k = 3
  Output: false
  Explanation: It is impossible to construct 3 palindromes using all the characters of s.
  
  Example 3:
  Input: s = "true", k = 4
  Output: true
  Explanation: The only possible solution is to put each character in a separate string.
*/
public class Solution {
    public bool CanConstruct(string s, int k) {
        if (s.Length < k) return false;

        Span<int> a = stackalloc int[26];
        foreach (char ch in s) {
            int i = ch - 'a';
            ++a[i];
        }

        int odd = 0;
        foreach (int i in a) {
            if (i % 2 != 0) ++odd;
        }

        return odd <= k;
    }
}
