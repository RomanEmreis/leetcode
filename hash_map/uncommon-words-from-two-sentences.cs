/*
  A sentence is a string of single-space separated words where each word consists only of lowercase letters.
  A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
  
  Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
  
  Example 1:
    Input: s1 = "this apple is sweet", s2 = "this apple is sour"
    Output: ["sweet","sour"]
    Explanation:
    The word "sweet" appears only in s1, while the word "sour" appears only in s2.
  
  Example 2:
    Input: s1 = "apple apple", s2 = "banana"
    Output: ["banana"]
*/
public class Solution {
    public string[] UncommonFromSentences(string s1, string s2) {
        Dictionary<string, int> counter1 = [];
        foreach (string w in s1.Split(' ')) {
            if (counter1.ContainsKey(w)) ++counter1[w];
            else counter1[w] = 1;
        }

        Dictionary<string, int> counter2 = [];
        foreach (string w in s2.Split(' ')) {
            if (counter2.ContainsKey(w)) ++counter2[w];
            else counter2[w] = 1;
        }

        List<string> result = [];
        foreach (var (k, v) in counter1) {
            if (v != 1) continue;
            if (counter2.ContainsKey(k)) continue;

            result.Add(k);
        }

        foreach (var (k, v) in counter2) {
            if (v != 1) continue;
            if (counter1.ContainsKey(k)) continue;

            result.Add(k);
        }

        return result.ToArray();
    }
}
