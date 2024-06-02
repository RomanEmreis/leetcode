/*
  Write a function that reverses a string. The input string is given as an array of characters s.
  You must do this by modifying the input array in-place with O(1) extra memory.
*/
class Solution {
public:
    void reverseString(vector<char>& s) {
        int n = s.size();
        for (int i = 0, j = n - 1; i < j; ++i, --j) {
            int tmp = s[i];
            s[i] = s[j];
            s[j] = tmp;
        }
    }
};
