/*
  You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.
  An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.
  
  Return the number of islands in grid2 that are considered sub-islands.

  Example 1:
    Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
    Output: 3
    Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
    The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

  Example 2:
    Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
    Output: 2 
    Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
    The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.
*/
auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();

class Solution {
public:
    int countSubIslands(vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        size_t m = grid2.size(), n = grid2[0].size();
        int result = 0;

        for (size_t i = 0; i < m; ++i) {
            for (size_t j = 0; j < n; ++j) {
                if (grid2[i][j] == 1 && dfs(grid1, grid2, i, j)) ++result;
            }
        }

        return result;
    }
private:
    static bool dfs(vector<vector<int>>& grid1, vector<vector<int>>& grid2, int i, int j) {
        size_t m = grid2.size(), n = grid2[0].size();
        if (i < 0 || j < 0 || i >= m || j >= n || grid2[i][j] == 0) return true;

        grid2[i][j] = 0;

        bool isSub = (grid1[i][j] == 1);

        isSub &= dfs(grid1, grid2, i + 1, j);
        isSub &= dfs(grid1, grid2, i - 1, j);
        isSub &= dfs(grid1, grid2, i, j + 1);
        isSub &= dfs(grid1, grid2, i, j - 1);

        return isSub;
    }
};
