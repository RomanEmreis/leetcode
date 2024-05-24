/*
  Given a list of words, list of  single letters (might be repeating) and score of every character.
  Return the maximum score of any valid set of words formed by using the given letters (words[i] cannot be used two or more times).

  It is not necessary to use all characters in letters and each letter can only be used once. Score of letters 'a', 'b', 'c', ... ,'z' is given by score[0], score[1], ... , score[25] respectively.

  Example:
    Input: words = ["dog","cat","dad","good"], letters = ["a","a","c","d","d","d","g","o","o"], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
    Output: 23
    Explanation:
    Score  a=1, c=9, d=5, g=3, o=2
    Given letters, we can form the words "dad" (5+1+5) and "good" (3+2+2+5) with a score of 23.
    Words "dad" and "dog" only get a score of 21.
*/
public class Solution {
    public int MaxScoreWords(string[] words, char[] letters, int[] score) {
        Span<int> a = stackalloc int[score.Length];
        Span<int> b = stackalloc int[score.Length];

        int result = 0;

        foreach (char c in letters) a[c - 'a']++;
        Dfs(0, ref a, ref b);

        return result;

        void Dfs(int idx, ref Span<int> a, ref Span<int> b) {
            if (idx == words.Length) {
                int sc = 0;
                for (int i = 0; i < score.Length; ++i) {
                    if (b[i] > a[i]) return;
                    sc += score[i] * b[i];
                }
                result = Math.Max(result, sc);
                return;
            }

            Dfs(idx + 1, ref a, ref b);
            foreach (char c in words[idx]) b[c - 'a']++;
            Dfs(idx + 1, ref a, ref b);
            foreach (char c in words[idx]) b[c - 'a']--;
        }
    }
}
