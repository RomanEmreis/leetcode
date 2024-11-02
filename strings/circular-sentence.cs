/*
  A sentence is a list of words that are separated by a single space with no leading or trailing spaces.
  
  For example, "Hello World", "HELLO", "hello world hello world" are all sentences.
  Words consist of only uppercase and lowercase English letters. Uppercase and lowercase English letters are considered different.
  
  A sentence is circular if:
  The last character of a word is equal to the first character of the next word.
  The last character of the last word is equal to the first character of the first word.
  For example, "leetcode exercises sound delightful", "eetcode", "leetcode eats soul" are all circular sentences. However, "Leetcode is cool", "happy Leetcode", "Leetcode" and "I like Leetcode" are not circular sentences.
  
  Given a string sentence, return true if it is circular. Otherwise, return false.
  
  Example 1:
    Input: sentence = "leetcode exercises sound delightful"
    Output: true
    Explanation: The words in sentence are ["leetcode", "exercises", "sound", "delightful"].
    - leetcode's last character is equal to exercises's first character.
    - exercises's last character is equal to sound's first character.
    - sound's last character is equal to delightful's first character.
    - delightful's last character is equal to leetcode's first character.
    The sentence is circular.
*/
public class Solution {
    public bool IsCircularSentence(string sentence) {
        if (sentence[0] != sentence[^1]) return false;

        string[] parts = sentence.Split(" ");
        if (parts.Length == 1) return true;

        for (int i = 1; i < parts.Length; ++i) {
            if (parts[i - 1][^1] != parts[i][0]) return false;
        }

        return true;
    }
}
