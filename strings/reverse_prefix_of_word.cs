public class Solution {
    public string ReversePrefix(string word, char ch) {
        if (word.Length == 1) return word;

        int idx = word.IndexOf(ch);
        if (idx == -1) return word;

        StringBuilder sb = new(word.Length);
        for (int i = idx; i >= 0; --i) sb.Append(word[i]);
        for (int i = idx + 1; i < word.Length; ++i) sb.Append(word[i]);

        return sb.ToString();
    }
}
