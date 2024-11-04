/*
  Given a string word, compress it using the following algorithm:
  
  Begin with an empty string comp. While word is not empty, use the following operation:
  Remove a maximum length prefix of word made of a single character c repeating at most 9 times.
  Append the length of the prefix followed by c to comp.
  Return the string comp.
  
  Example 1:
    Input: word = "abcde"
    Output: "1a1b1c1d1e"
    Explanation:
    Initially, comp = "". Apply the operation 5 times, choosing "a", "b", "c", "d", and "e" as the prefix in each operation.
    For each prefix, append "1" followed by the character to comp.
  
  Example 2:
    Input: word = "aaaaaaaaaaaaaabb"
    Output: "9a5a2b"
*/
public class Solution {
    public string CompressedString(string word) {
        StringBuilder comp = new();

        int counter = 1;
        for (int i = 1; i < word.Length; ++i) {
            if (word[i] == word[i - 1]) {
                ++counter;
            } else {
                while (counter > 9) {
                    comp.Append($"9{word[i - 1]}");
                    counter -= 9;
                }
                comp.Append($"{counter}{word[i - 1]}");
                counter = 1;
            }
        }

        while (counter > 9) {
            comp.Append($"9{word[^1]}");
            counter -= 9;
        }
        comp.Append($"{counter}{word[^1]}");

        return comp.ToString();
    }
}
