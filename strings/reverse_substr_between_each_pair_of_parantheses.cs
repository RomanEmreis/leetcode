/*
  You are given a string s that consists of lower case English letters and brackets.
  Reverse the strings in each pair of matching parentheses, starting from the innermost one.
  Your result should not contain any brackets.

  Example:
    Input: s = "(abcd)"
    Output: "dcba"
*/
public class Solution {
    public string ReverseParentheses(string s) {
        Stack<char> st = [];
        Queue<char> q = [];

        foreach (char c in s) {
            if (c != ')') st.Push(c);
            else {
                while (st.TryPop(out char c1) && c1 != '(') q.Enqueue(c1);
                while (q.TryDequeue(out char c2)) st.Push(c2);
            }
        }

        Span<char> result = stackalloc char[st.Count];
        for (int i = st.Count - 1; i >= 0; --i) {
            result[i] = st.Pop();
        }
        return new string(result);
    }
}
