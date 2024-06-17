/*
  Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.

  Example:
    Input: c = 5
    Output: true
    Explanation: 1 * 1 + 2 * 2 = 5
*/
func judgeSquareSum(c int) bool {
    c64 := int64(c);
    l, r := int64(0), int64(math.Sqrt(float64(c)));
    for l <= r {
        x := l * l + r * r;
        if x == c64 {
            return true;
        } else if x > c64 {
            r--;
        } else {
            l++;
        }
    }

    return false;
}
