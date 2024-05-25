/*
  Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
  Note that the same word in the dictionary may be reused multiple times in the segmentation.

  Example:
    Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
    Output: ["cats and dog","cat sand dog"]
*/
public class Solution {
    public IList<string> WordBreak(string s, IList<string> wordDict) {
        HashSet<string> set = [..wordDict];
        return Dfs(s, 0, set);
    }

    private static List<string> Dfs(string s, int start, HashSet<string> set) {
        List<string> result = [];

        if (start == s.Length) result.Add("");

        for (int end = start + 1; end <= s.Length; ++end) {
            string prefix = s[start..end];
            if (set.Contains(prefix)) {
                List<string> suffixes = Dfs(s, end, set);
                foreach (string suffix in suffixes) {
                    result.Add(prefix + (suffix == "" ? "" : " ") + suffix);
                }
            }
        }

        return result;
    }
}
