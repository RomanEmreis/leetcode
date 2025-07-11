/*
  1061. Lexicographically Smallest Equivalent String
  
  You are given two strings of the same length s1 and s2 and a string baseStr.
  We say s1[i] and s2[i] are equivalent characters.
      For example, if s1 = "abc" and s2 = "cde", then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.
  
  Equivalent characters follow the usual rules of any equivalence relation:
      Reflexivity: 'a' == 'a'.
      Symmetry: 'a' == 'b' implies 'b' == 'a'.
      Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.
  
  For example, given the equivalency information from s1 = "abc" and s2 = "cde", "acd" and "aab" are equivalent strings
  of baseStr = "eed", and "aab" is the lexicographically smallest equivalent string of baseStr.
  
  Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.
  
  Example 1:
  Input: s1 = "parker", s2 = "morris", baseStr = "parser"
  Output: "makkek"
  Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
  The characters in each group are equivalent and sorted in lexicographical order.
  So the answer is "makkek".
  
  Example 2:
  Input: s1 = "hello", s2 = "world", baseStr = "hold"
  Output: "hdld"
  Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
  So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".
  
  Example 3:
  Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
  Output: "aauaaaaada"
  Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], 
  thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".
*/
public class Solution {
    public string SmallestEquivalentString(string s1, string s2, string baseStr) {
        Span<int> p = stackalloc int[26];
        for (int i = 0; i < 26; ++i) p[i] = i;

        for (int i = 0; i < s1.Length; ++i) {
            int a = s1[i] - 'a';
            int b = s2[i] - 'a';

            int pa = Find(a, ref p);
            int pb = Find(b, ref p);

            if (pa < pb) p[pb] = pa;
            else p[pa] = pb;
        }

        Span<char> result = stackalloc char[baseStr.Length];
        for (int i = 0; i < result.Length; ++i) {
            int idx = Find(baseStr[i] - 'a', ref p);
            result[i] = (char) (idx + 'a');
        }

        return new string(result);

        int Find(int x, ref Span<int> p) {
            if (x != p[x]) p[x] = Find(p[x], ref p);
            return p[x];
        }
    }
}
