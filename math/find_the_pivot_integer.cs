public class Solution {
    public int PivotInteger(int n) {
        if (n == 1) return 1;

        double x = Math.Sqrt(n * (n + 1) / 2);
        return x % 1 == 0 ? (int) x : -1;
    }
}
