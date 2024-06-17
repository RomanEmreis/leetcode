/*
  Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.

  Example:
    Input: c = 5
    Output: true
    Explanation: 1 * 1 + 2 * 2 = 5
*/
class Solution {
public:
    bool judgeSquareSum(int c) {
        long long l = 0, r = sqrt(c);
        while (l <= r) {
            long long x = l * l + r * r;
            if (x == c) return true;
            else if (x > c) --r;
            else ++l;
        }

        return false;
    }
};
