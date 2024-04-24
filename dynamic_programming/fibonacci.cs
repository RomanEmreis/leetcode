public class Solution {
    public int Fib(int n) {
        if (n is 0 or 1) return n;

        int a = 0, b = 1, result = 0;
        for (int i = 2; i <= n; ++i) {
            result = a + b;
            (a, b) = (b, result);
        }

        return result;
    }
}
