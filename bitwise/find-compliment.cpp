class Solution {
public:
    int findComplement(int num) {
        size_t n = 1;
        while (n <= num) n *= 2;
        return n - 1 - num;
    }
};
