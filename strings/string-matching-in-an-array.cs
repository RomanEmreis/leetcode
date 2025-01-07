/*
  1408. String Matching in an Array
  
  Given an array of string words, return all strings in words that is a substring of another word. You can return the answer in any order.
  A substring is a contiguous sequence of characters within a string
  
  Example 1:
  Input: words = ["mass","as","hero","superhero"]
  Output: ["as","hero"]
  Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
  ["hero","as"] is also a valid answer.
  
  Example 2:
  Input: words = ["leetcode","et","code"]
  Output: ["et","code"]
  Explanation: "et", "code" are substring of "leetcode".
  
  Example 3:
  Input: words = ["blue","green","bu"]
  Output: []
  Explanation: No string of words is substring of another string.
*/
public class Solution {
    public IList<string> StringMatching(string[] words) {
        HashSet<string> result = [];
        for (int i = 0; i < words.Length; ++i) {
            for (int j = i + 1; j < words.Length; ++j) {
                if (words[j].Contains(words[i])) result.Add(words[i]);
                if (words[i].Contains(words[j])) result.Add(words[j]);
            }
        }
        return result.ToArray();
    }
}
