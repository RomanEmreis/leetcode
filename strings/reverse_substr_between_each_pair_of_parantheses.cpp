/*
  You are given a string s that consists of lower case English letters and brackets.
  Reverse the strings in each pair of matching parentheses, starting from the innermost one.
  Your result should not contain any brackets.

  Example:
    Input: s = "(abcd)"
    Output: "dcba"
*/
class Solution {
public:
    string reverseParentheses(string s) {
        stack<char> st;
        queue<char> q;

        for (const char& c : s) {
            if (c != ')') st.push(c);
            else {
                for (;!st.empty() && st.top() != '('; st.pop()) q.push(st.top());
                if (!st.empty()) st.pop();
                for (;!q.empty(); q.pop()) st.push(q.front());
            }
        }

        string result;
        while (!st.empty()) {
            result = st.top() + result;
            st.pop();
        }
        return result;
    }
};
