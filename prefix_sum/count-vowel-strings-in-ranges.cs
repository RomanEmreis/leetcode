/*
  2559. Count Vowel Strings in Ranges
  
  You are given a 0-indexed array of strings words and a 2D array of integers queries.
  Each query queries[i] = [li, ri] asks us to find the number of strings present in the range li to ri (both inclusive) of words that start and end with a vowel.
  Return an array ans of size queries.length, where ans[i] is the answer to the ith query.
  
  Note that the vowel letters are 'a', 'e', 'i', 'o', and 'u'.
  
  Example 1:
  Input: words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
  Output: [2,3,0]
  Explanation: The strings starting and ending with a vowel are "aba", "ece", "aa" and "e".
  The answer to the query [0,2] is 2 (strings "aba" and "ece").
  to query [1,4] is 3 (strings "ece", "aa", "e").
  to query [1,1] is 0.
  We return [2,3,0].
  
  Example 2:
  Input: words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
  Output: [3,2,1]
  Explanation: Every string satisfies the conditions, so we return [3,2,1].
*/
public class Solution {
    public int[] VowelStrings(string[] words, int[][] queries) {
        Span<int> prefix = stackalloc int[words.Length + 1];
        int count = 0;
        for (int i = 0; i < words.Length; ++i) {
            if (IsVowel(words[i][0]) && IsVowel(words[i][^1])) ++count;
            prefix[i + 1] = count;
        }

        int[] result = new int[queries.Length];
        for (int i = 0; i < queries.Length; ++i) {
            int start = queries[i][0];
            int end = queries[i][1] + 1;
            result[i] = prefix[end] - prefix[start];
        }

        return result;

        static bool IsVowel(char ch) =>
            ch == 'a'
            || ch == 'e'
            || ch == 'i'
            || ch == 'o'
            || ch == 'u';
    }
}
