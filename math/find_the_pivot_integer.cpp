class Solution {
public:
    int pivotInteger(int n) {
        if (n == 1) return 1;

        double x = sqrt(n * (n + 1) / 2);
        return x == (int) x ? x : -1;
    }
};
