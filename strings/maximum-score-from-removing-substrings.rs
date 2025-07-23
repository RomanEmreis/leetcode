/*
1717. Maximum Score From Removing Substrings

You are given a string s and two integers x and y. You can perform two types of operations any number of times.
Remove substring "ab" and gain x points.
  For example, when removing "ab" from "cabxbae" it becomes "cxbae".
Remove substring "ba" and gain y points.
  For example, when removing "ba" from "cabxbae" it becomes "cabxe".

Return the maximum points you can gain after applying the above operations on s.

Example 1:
Input: s = "cdbcbbaaabab", x = 4, y = 5
Output: 19
Explanation:
- Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
- Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
- Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
- Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
Total score = 5 + 4 + 5 + 5 = 19.

Example 2:
Input: s = "aabbaaxybbaabb", x = 5, y = 4
Output: 20
*/
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (top, top_score, bot, bot_score) = if y > x {
            (['b', 'a'], y, ['a', 'b'], x)
        } else {
            (['a', 'b'], x, ['b', 'a'], y)
        };

        let mut res = 0;
        let mut st = Vec::with_capacity(s.len());
        for ch in s.chars() {
            if ch == top[1] && st.last() == Some(&top[0]) {
                st.pop();
                res += top_score;
            } else {
                st.push(ch);
            }
        }

        let mut st1 = Vec::with_capacity(st.len());
        for ch in st {
            if ch == bot[1] && st1.last() == Some(&bot[0]) {
                st1.pop();
                res += bot_score;
            } else {
                st1.push(ch);
            }
        }

        res
    }
}
