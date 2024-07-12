/*
  You are given a string s and two integers x and y. You can perform two types of operations any number of times.
  
  Remove substring "ab" and gain x points.
  For example, when removing "ab" from "cabxbae" it becomes "cxbae".
  Remove substring "ba" and gain y points.
  For example, when removing "ba" from "cabxbae" it becomes "cabxe".
  Return the maximum points you can gain after applying the above operations on s.

  Example:
    Input: s = "cdbcbbaaabab", x = 4, y = 5
    Output: 19
    Explanation:
    - Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
    - Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
    - Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
    - Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
    Total score = 5 + 4 + 5 + 5 = 19.
*/
public class Solution {
    public int MaximumGain(string s, int x, int y) {
        int result = 0;

        string top, bot;
        int top_score, bot_score;

        if (x > y) {
            top = "ab";
            top_score = x;
            bot = "ba";
            bot_score = y;
        } else {
            top = "ba";
            top_score = y;
            bot = "ab";
            bot_score = x;
        }

        List<char> st = [];
        foreach (char ch in s) {
            if (ch == top[1] && st.Count > 0 && st[^1] == top[0]) {
                result += top_score;
                st.RemoveAt(st.Count - 1);
            } else {
                st.Add(ch);
            }
        }

        List<char> st2 = [];
        foreach (char ch in st) {
            if (ch == bot[1] && st2.Count > 0 && st2[^1] == bot[0]) {
                result += bot_score;
                st2.RemoveAt(st2.Count - 1);
            } else {
                st2.Add(ch);
            }
        }

        return result;
    }
}
