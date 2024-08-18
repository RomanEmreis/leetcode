/*
  An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
  Given an integer n, return the nth ugly number.

  Example 1:
    Input: n = 10
    Output: 12
    Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
  
  Example 2:
    Input: n = 1
    Output: 1
    Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
*/
func nthUglyNumber(n int) int {
    dp := make([]int, n);
    dp[0] = 1;

    i, i2, i3, i5 := 1, 0, 0, 0;
    f2, f3, f5 := 2, 3, 5;
    for i < n {
        m := min(f2, min(f3, f5));
        dp[i] = m;
        i++;

        if m == f2 {
            i2++;
            f2 = 2 * dp[i2];
        }
        if m == f3 {
            i3++;
            f3 = 3 * dp[i3];
        }
        if m == f5 {
            i5++;
            f5 = 5 * dp[i5];
        } 
    }

    return dp[n - 1];
}

func min(x, y int) int {
    if x > y {
        return y;
    }
    return x;
}
