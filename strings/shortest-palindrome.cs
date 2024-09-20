/*
  You are given a string s. You can convert s to a palindrome by adding characters in front of it.
  
  Return the shortest palindrome you can find by performing this transformation.
  
  Example 1:
  Input: s = "aacecaaa"
  Output: "aaacecaaa"
  
  Example 2:
  Input: s = "abcd"
  Output: "dcbabcd"
*/
public class Solution {
    public string ShortestPalindrome(string s) {
        if (string.IsNullOrEmpty(s)) return s;
        
        string reversed = string.Join("", s.Reverse());
        string str = s + "#" + reversed;

        int n = str.Length;
        Span<int> lps = stackalloc int[n];

        CalculateLPS(str, ref lps);
        
        int len = lps[lps.Length - 1];
        string prefix = reversed[0..(s.Length - len)];
        
        return prefix + s;   
    }

    private static void CalculateLPS(string str, ref Span<int> lps) {
        int n = str.Length;
        int i = 1, len = 0;
        lps[0] = 0;
        
        while (i < n) {
            if (str[i] == str[len]) {
                len++;
                lps[i] = len;
                i++;
            } else {
                if (len > 0) {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i++;
                }
            }
        }
    }
}
