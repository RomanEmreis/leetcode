/*
  Given a string s, return the maximum number of unique substrings that the given string can be split into.
  You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.
  
  A substring is a contiguous sequence of characters within a string.
  
  Example 1:
    Input: s = "ababccc"
    Output: 5
    Explanation: One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.
  
  Example 2:
    Input: s = "aba"
    Output: 2
    Explanation: One way to split maximally is ['a', 'ba'].
  
  Example 3:
    Input: s = "aa"
    Output: 1
    Explanation: It is impossible to split the string any further.
*/
public class Solution {
    public int MaxUniqueSplit(string s) {
        int result = 0;
        HashSet<string> visited = [];

        Backtrack(s, 0);

        return result;

        void Backtrack(string s, int index) {
            if (index == s.Length) {
                if (result < visited.Count) result = visited.Count;
                return;
            }

            for (int i = index; i < s.Length; ++i) {
                string sub = s[index..(i + 1)];
                if (visited.Add(sub)) {
                    Backtrack(s, i + 1);
                    visited.Remove(sub);
                }
            }
        }
    }
}
