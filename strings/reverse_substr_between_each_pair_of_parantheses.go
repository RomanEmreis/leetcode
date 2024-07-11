/*
  You are given a string s that consists of lower case English letters and brackets.
  Reverse the strings in each pair of matching parentheses, starting from the innermost one.
  Your result should not contain any brackets.

  Example:
    Input: s = "(abcd)"
    Output: "dcba"
*/
func reverseParentheses(s string) string {
    st := []rune{};

    for _, c := range s {
        if c != ')' {
            st = append(st, c);
        } else {
            q := []rune{};
            for len(st) > 0 && st[len(st) - 1] != '(' {
                q = append(q, st[len(st) - 1]);
                st = st[:len(st) - 1];
            }
            if len(st) > 0 {
                st = st[:len(st) - 1];
            }
            for _, c1 := range q {
                st = append(st, c1);
            }
        }
    }

    return string(st);
}
