/*
  Given a string s, partition s such that every substring of the partition is a palindrome
  Return all possible palindrome partitioning of s.

  Example:
    Input: s = "aab"
    Output: [["a","a","b"],["aa","b"]]
*/
public class Solution {
    public IList<IList<string>> Partition(string s) {
        if (s.Length == 0) return [];
        if (s.Length == 1) return [[s]];

        IList<IList<string>> result = [];

        for (int i = 1; i <= s.Length; ++i) {
            string sub = s[..i];
            if (isPalindrome(sub)) {
                if (i == s.Length) {
                    result.Add([sub]);
                    continue;
                }

                foreach (var p in Partition(s[i..])) {
                    List<string> list = [sub];
                    list.AddRange(p);
                    result.Add(list);
                }
            }
        }

        return result;
    }

    private static bool isPalindrome(string s) {
        for (int i = 0; i < s.Length / 2; ++i) {
            if (s[i] != s[s.Length - i - 1]) return false;
        }

        return true;
    }
}
