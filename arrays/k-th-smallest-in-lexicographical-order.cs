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
public class Solution {
    public int FindKthNumber(int n, int k) {       
        long current = 1;
        k -= 1; 
        while (k > 0) {
            long steps = 0;
            long first = current;
            long last = current;
            while (first <= n) {
                if (last < n) steps += last - first + 1;
                else steps += n - first + 1;
                
                first *= 10;
                last = last * 10 + 9;
            }
            if (steps <= k) {
                current += 1;
                k -= (int)steps;
            } else {
                current *= 10;
                k -= 1; 
            }
        }
        
        return (int)current;
    }
}
