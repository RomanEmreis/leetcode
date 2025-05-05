/*
  790. Domino and Tromino Tiling
  
  You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
  Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.
  
  In a tiling, every square must be covered by a tile. Two tilings are different 
  if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
  
  Example 1:
  Input: n = 3
  Output: 5
  Explanation: The five different ways are show above.
  
  Example 2:
  Input: n = 1
  Output: 1
*/
public class Solution {
    public int NumTilings(int n) {
        const int mod = (int)1e9 + 7;
        Span<long> dp = [1,0,0,0];
        for (int i = 1; i <= n; ++i) {
            Span<long> t = stackalloc long[4];
            t[0] = (dp[0] + dp[1] + dp[2] + dp[3]) % mod;
            t[1] = (dp[2] + dp[3]) % mod;
            t[2] = (dp[1] + dp[3]) % mod;
            t[3] = dp[0];
            dp = t;
        }
        return (int)dp[0];
    }
}
