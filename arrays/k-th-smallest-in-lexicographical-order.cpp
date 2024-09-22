/*
  Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].
  
  Example 1:
    Input: n = 13, k = 2
    Output: 10
    Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.

  Example 2:
    Input: n = 1, k = 1
    Output: 1
*/
class Solution {
public:
    int findKthNumber(int n, int k) {
        long long curr = 1;
        --k;

        while (k > 0) {
            long long steps = 0, first = curr, last = curr;
            while (first <= n) {
                steps += last < n ? last - first + 1 : n - first + 1;
                first *= 10;
                last = last * 10 + 9;
            }
            if (steps <= k) {
                ++curr;
                k -= steps;
            } else {
                curr *= 10;
                --k;
            }
        }

        return (int) curr;
    }
};
