/*
  3442. Maximum Difference Between Even and Odd Frequency I
  
  You are given a string s consisting of lowercase English letters.
  
  Your task is to find the maximum difference diff = freq(a1) - freq(a2) between the frequency of characters a1 and a2 in the string such that:
    a1 has an odd frequency in the string.
    a2 has an even frequency in the string.
  
  Return this maximum difference.
  
  Example 1:
  Input: s = "aaaaabbc"
  Output: 3
  Explanation:
  The character 'a' has an odd frequency of 5, and 'b' has an even frequency of 2.
  The maximum difference is 5 - 2 = 3.
  
  Example 2:
  Input: s = "abcabcab"
  Output: 1
  Explanation:
  The character 'a' has an odd frequency of 3, and 'c' has an even frequency of 2.
  The maximum difference is 3 - 2 = 1.
*/
public class Solution {
    public int MaxDifference(string s) {
        Span<int> count = stackalloc int[26];
        foreach (char ch in s) ++count[ch - 'a'];

        int maxOdd = 0;
        int minEven = int.MaxValue;

        foreach (int c in count) {
            if (c == 0) continue;
            if (c % 2 == 0) minEven = minEven > c ? c : minEven;
            else maxOdd = maxOdd < c ? c : maxOdd;
        }

        return maxOdd - minEven;
    }
}
