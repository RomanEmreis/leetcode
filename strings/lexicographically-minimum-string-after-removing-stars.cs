/*
  3170. Lexicographically Minimum String After Removing Stars
  
  You are given a string s. It may contain any number of '*' characters. Your task is to remove all '*' characters.
  While there is a '*', do the following operation:
      Delete the leftmost '*' and the smallest non-'*' character to its left. If there are several smallest characters, you can delete any of them.
  
  Return the resulting string after removing all '*' characters.
  
  Example 1:
  Input: s = "aaba*"
  Output: "aab"
  Explanation:
  We should delete one of the 'a' characters with '*'. If we choose s[3], s becomes the lexicographically smallest.
  
  Example 2:
  Input: s = "abc"
  Output: "abc"
  Explanation:
  There is no '*' in the string.
*/
public class Solution {
    public string ClearStars(string s) {
        StringBuilder sb = new(s);
        var st = new Stack<int>[26];

        for (int i = 0; i < s.Length; ++i) {
            if (s[i] == '*') {
                for (int j = 0; j < 26; ++j) {
                    if (st[j] is { Count: > 0 }) {
                        sb[st[j].Pop()] = '*';
                        break;
                    }
                }
            } else {
                int idx = s[i] - 'a';
                st[idx] ??= [];
                st[idx].Push(i);
            }
        }

        return sb.Replace("*", "").ToString();
    }
}
