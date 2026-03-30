/*
  2840. Check if Strings Can be Made Equal With Operations II
  
  You are given two strings s1 and s2, both of length n, consisting of lowercase English letters.
  You can apply the following operation on any of the two strings any number of times:
      Choose any two indices i and j such that i < j and the difference j - i is even, then swap the two characters at those indices in the string.
  
  Return true if you can make the strings s1 and s2 equal, and false otherwise.
  
  Example 1:
  Input: s1 = "abcdba", s2 = "cabdab"
  Output: true
  Explanation: We can apply the following operations on s1:
  - Choose the indices i = 0, j = 2. The resulting string is s1 = "cbadba".
  - Choose the indices i = 2, j = 4. The resulting string is s1 = "cbbdaa".
  - Choose the indices i = 1, j = 5. The resulting string is s1 = "cabdab" = s2.
  
  Example 2:
  Input: s1 = "abe", s2 = "bea"
  Output: false
  Explanation: It is not possible to make the two strings equal.
*/
public class Solution {
    public bool CheckStrings(string s1, string s2) {
        Span<int> even = stackalloc int[26];
        Span<int> odd = stackalloc int[26];

        for (int i = 0; i < s1.Length; ++i) {
            if (i % 2 == 0) {
                ++even[s1[i] - 'a'];
                --even[s2[i] - 'a'];
            } else {
                ++odd[s1[i] - 'a'];
                --odd[s2[i] - 'a'];
            }
                
        }

        foreach (int n in even)
            if (n != 0)
                return false;

        foreach (int n in odd)
            if (n != 0)
                return false;

        return true;
    }
}
