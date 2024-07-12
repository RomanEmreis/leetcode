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
func pop(s []rune) []rune {
    return s[:len(s) - 1];
}

func maximumGain(s string, x int, y int) int {
    result := 0;

    var top, bot string;
    var top_score, bot_score int;

    if x > y {
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

    ss := []rune(s);
    st := make([]rune, 0, len(ss));

    for _, ch := range ss {
        if ch == rune(top[1]) && len(st) > 0 && st[len(st)-1] == rune(top[0]) {
            result += top_score;
            st = pop(st);
        } else {
            st = append(st, ch);
        }
    }

    st2 := make([]rune, 0, len(st));
    for _, ch := range st {
        if ch == rune(bot[1]) && len(st2) > 0 && st2[len(st2)-1] == rune(bot[0]) {
            result += bot_score;
            st2 = pop(st2);
        } else {
            st2 = append(st2, ch);
        }
    }

    return result;
}
