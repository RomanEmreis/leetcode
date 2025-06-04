/*
  3403. Find the Lexicographically Largest String From the Box I
  
  You are given a string word, and an integer numFriends.
  Alice is organizing a game for her numFriends friends. There are multiple rounds in the game, where in each round:
      word is split into numFriends non-empty strings, such that no previous round has had the exact same split.
      All the split words are put into a box.
  
  Find the string from the box after all the rounds are finished.
  
  Example 1:
  Input: word = "dbca", numFriends = 2
  Output: "dbc"
  Explanation: 
  All possible splits are:
      "d" and "bca".
      "db" and "ca".
      "dbc" and "a".
  
  Example 2:
  Input: word = "gggg", numFriends = 4
  Output: "g"
  Explanation: 
  The only possible split is: "g", "g", "g", and "g".
*/
public class Solution {
    public string AnswerString(string word, int numFriends) {
        if (numFriends == 1) return word;

        int i = 0;
        int j = 1;
        int k = 0;

        while (j + k < word.Length) {
            if (word[i + k] == word[j + k]) {
                ++k;
            } else if (word[i + k] > word[j + k]) {
                j = j + k + 1;
                k = 0;
            } else {
                i = Math.Max(i + k + 1, j);
                j = i + 1;
                k = 0;
            }
        }

        string str = word[i..];
        int size = Math.Min(str.Length, word.Length - numFriends + 1);

        return str[..size];
    }
}
