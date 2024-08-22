public class Solution {
    public int FindComplement(int num) {
        uint n = 1;
        while (n <= num) n *= 2;

        return (int)n - 1 - num;
    }
}
