/*
  Alice and Bob continue their games with piles of stones.  There are a number of piles arranged in a row, and each pile has a positive integer number of stones piles[i].  The objective of the game is to end with the most stones. 
  Alice and Bob take turns, with Alice starting first.  Initially, M = 1.
  On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M.  Then, we set M = max(M, X).
  
  The game continues until all the stones have been taken.
  
  Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.
  
  Example 1:
    Input: piles = [2,7,9,4,4]
    Output: 10
    Explanation:  If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 piles in total. If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger. 
    
  Example 2:
    Input: piles = [1,2,3,4,5,100]
    Output: 104
*/
class Solution {
private:
    int helper(int i, int M, vector<int>& piles, vector<vector<int>>& dp, vector<int>& suffixSum) {
        size_t n = piles.size();

        if (i >= n) return 0;
        if (dp[i][M] != -1) return dp[i][M];

        int maxStones = 0;

        for (int X = 1; X <= 2 * M; ++X) {
            if (i + X > n) break; 
            int opponentScore = helper(i + X, max(M, X), piles, dp, suffixSum);
            maxStones = max(maxStones, suffixSum[i] - opponentScore);
        }

        dp[i][M] = maxStones;
        return dp[i][M];
    }

public:
    int stoneGameII(vector<int>& piles) {
        size_t n = piles.size();
        vector<vector<int>> dp(n, vector<int>(n + 1, -1));
        vector<int> suffixSum(n, 0);

        suffixSum[n - 1] = piles[n - 1];
        for (int i = n - 2; i >= 0; --i) {
            suffixSum[i] = suffixSum[i + 1] + piles[i];
        }
        return helper(0, 1, piles, dp, suffixSum);
    }
};