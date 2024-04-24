public class Solution {
    public int Tribonacci(int n) {
        if (n == 0) return 0;
        if (n == 1 || n == 2) return 1;

        int a = 0, b = 1, c = 1, result = 0;
        for (int i = 2; i < n; ++i) {
            result = a + b + c;
            (a, b, c) = (b, c, result);
        }

        return result;
    }
}
