/*
  2338. Count the Number of Ideal Arrays
  
  You are given two integers n and maxValue, which are used to describe an ideal array.
  
  A 0-indexed integer array arr of length n is considered ideal if the following conditions hold:
  Every arr[i] is a value from 1 to maxValue, for 0 <= i < n.
  Every arr[i] is divisible by arr[i - 1], for 0 < i < n.
  
  Return the number of distinct ideal arrays of length n. Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: n = 2, maxValue = 5
  Output: 10
  Explanation: The following are the possible ideal arrays:
  - Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
  - Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
  - Arrays starting with the value 3 (1 array): [3,3]
  - Arrays starting with the value 4 (1 array): [4,4]
  - Arrays starting with the value 5 (1 array): [5,5]
  There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.
  
  Example 2:
  Input: n = 5, maxValue = 3
  Output: 11
  Explanation: The following are the possible ideal arrays:
  - Arrays starting with the value 1 (9 arrays): 
     - With no other distinct values (1 array): [1,1,1,1,1] 
     - With 2nd distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
     - With 2nd distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
  - Arrays starting with the value 2 (1 array): [2,2,2,2,2]
  - Arrays starting with the value 3 (1 array): [3,3,3,3,3]
  There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.
*/
public class Solution {
    public int IdealArrays(int n, int maxValue) {
        const int mod = (int) 1e9 + 7;

        int[,] dp = new int[maxValue + 1, 16];
        int[,] count = new int[n, 16];

        for (int i = 0; i <= maxValue; ++i) {
            for (int j = 0; j < 16; ++j) {
                dp[i, j] = -1;
            }
        }

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j <= i && j < 16; ++j) {
                count[i, j] = j == 0 ? 1 : (count[i - 1, j] + count[i - 1, j - 1]) % mod;
            }
        }

        int result = 0;

        for (int i = 1; i <= maxValue; ++i) {
            result = (result + Dfs(i, 1)) % mod;
        }

        return result;

        int Dfs(int i, int c) {
            if (dp[i, c] != -1) return dp[i, c];

            int result = count[n - 1, c - 1];
            if (c < n) {
                for (int k = 2; k * i <= maxValue; ++k) {
                    result = (result + Dfs(k * i, c + 1)) % mod;
                }
            }

            dp[i, c] = result;
            return result;
        }
    }
}
