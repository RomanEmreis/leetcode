/*
  Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
  
  You must write an algorithm that runs in O(n) time and uses O(1) extra space. 
  
  Example 1:
  Input: n = 13
  Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
*/
class Solution {
public:
    vector<int> lexicalOrder(int n) {
        vector<int> result(n);
        int curr = 1;

        for (size_t i = 1; i <= n; ++i) {
            result[i - 1] = curr;
            if (curr * 10 <= n) curr *= 10;
            else {
                if (curr >= n) curr /= 10;
                ++curr;
                while (curr % 10 == 0) curr /= 10;
            }
        }

        return result;
    }
};
