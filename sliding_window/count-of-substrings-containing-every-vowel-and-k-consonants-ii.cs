/*
  3306. Count of Substrings Containing Every Vowel and K Consonants II
  
  You are given a string word and a non-negative integer k.
  
  Return the total number of substrings of word that contain every vowel ('a', 'e', 'i', 'o', and 'u') at least once and exactly k consonants.
  
  Example 1:
  Input: word = "aeioqq", k = 1
  Output: 0
  Explanation:
  There is no substring with every vowel.
  
  Example 2:
  Input: word = "aeiou", k = 0
  Output: 1
  Explanation:
  The only substring with every vowel and zero consonants is word[0..4], which is "aeiou".
  
  Example 3:
  Input: word = "ieaouqqieaouqq", k = 1
  Output: 3
  Explanation:
  The substrings with every vowel and one consonant are:
  word[0..5], which is "ieaouq".
  word[6..11], which is "qieaou".
  word[7..12], which is "ieaouq".
*/
public class Solution {
    public long CountOfSubstrings(string word, int k) {
        return CountOfSubstringsImpl(word, k) - CountOfSubstringsImpl(word, k + 1);

        static long CountOfSubstringsImpl(string word, int k) {
            long result = 0;
            Dictionary<char, int> count = [];

            int l = 0;
            int x = 0;

            foreach (char ch in word) {
                if (IsVowel(ch)) {
                    if (count.ContainsKey(ch)) ++count[ch];
                    else count[ch] = 1;
                } else {
                    ++x;
                }

                while (x >= k && count.Count == 5) {
                    char d = word[l++];
                    if (IsVowel(d)) {
                        if (--count[d] == 0) count.Remove(d);
                    } else --x;
                }
                result += l;
            }

            return  result;
        }

        static bool IsVowel(char ch) =>
            ch is 'a' or 'e' or 'i' or 'o' or 'u'; 
    }
}
