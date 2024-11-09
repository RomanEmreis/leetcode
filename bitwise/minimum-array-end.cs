/*
  You are given two integers n and x. You have to construct an array of positive integers nums of size n where for every 0 <= i < n - 1, nums[i + 1] is greater than nums[i], 
  and the result of the bitwise AND operation between all elements of nums is x.
  
  Return the minimum possible value of nums[n - 1].
  
  Example 1:
    Input: n = 3, x = 4
    Output: 6
    Explanation:
    nums can be [4,5,6] and its last element is 6.
*/
public class Solution {
    public long MinEnd(int n, int x) {
        --n;
        long result = x;

        for (int i = 0; i < 31; ++i) {
            if ((x >> i & 1) == 0) {
                result |= (n & 1) << i;
                n >>= 1;
            }
        }

        result |= (long) n << 31;

        return result;
    }
}
