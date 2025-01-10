/*
  916. Word Subsets
  
  You are given two string arrays words1 and words2.
  
  A string b is a subset of string a if every letter in b occurs in a including multiplicity.
  
  For example, "wrr" is a subset of "warrior" but is not a subset of "world".
  A string a from words1 is universal if for every string b in words2, b is a subset of a.
  
  Return an array of all the universal strings in words1. You may return the answer in any order.
  
  Example 1:
  Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["e","o"]
  Output: ["facebook","google","leetcode"]
  
  Example 2:
  Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["l","e"]
  Output: ["apple","google","leetcode"]
*/
public class Solution {
    public IList<string> WordSubsets(string[] words1, string[] words2) {
        Span<int> count = stackalloc int[26];
        foreach (string word in words2) {
            int[] c = Count(word);
            for (int i = 0; i < 26; ++i) {
                count[i] = Math.Max(count[i], c[i]);
            }
        }

        List<string> result = [];
        foreach (string word in words1) {
            int[] c = Count(word);
            if (IsUniversal(c, ref count)) {
                result.Add(word);
            }
        }
        return result;

        static int[] Count(string word) {
            int[] count = new int[26];
            foreach (char ch in word) {
                int i = ch - 'a';
                ++count[i];
            }
            return count;
        }

        static bool IsUniversal(int[] c1, ref Span<int> c2) {
            for (int i = 0; i < 26; ++i) {
                if (c1[i] < c2[i]) return false;
            }
            return true;
        }
    }
}
