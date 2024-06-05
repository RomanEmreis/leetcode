/*
  Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.

  Example:
    Input: words = ["bella","label","roller"]
    Output: ["e","l","l"]
*/
public class Solution {
    public IList<string> CommonChars(string[] words) {
        if (words.Length == 1) return words[0].Select(c => c.ToString()).ToList();

        IEnumerable<char> result = words[0].ToList();

        for (int i = 1; i < words.Length; ++i) {
            List<char> w = words[i].ToList();
            result = result.Where(c => w.Remove(c));
        }

        return result.Select(c => c.ToString()).ToList();
    }
}
