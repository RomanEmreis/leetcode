/*
  A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9 such that each row, column, and both diagonals all have the same sum.
  Given a row x col grid of integers, how many 3 x 3 contiguous magic square subgrids are there?
  Note: while a magic square can only contain numbers from 1 to 9, grid may contain numbers up to 15.
*/
class Solution {
private:
    bool isMagic(const vector<vector<int>>& grid, const int& x, const int& y) {
        unordered_set<int> unique_numbers;
        
        for (int i = x; i < x + 3; i++) {
            for (int j = y; j < y + 3; j++) {
                int num = grid[i][j];
                if (num < 1 || num > 9 || unique_numbers.contains(num)) return false;
                unique_numbers.insert(num);
            }
        }

        int sum = grid[x][y] + grid[x][y + 1] + grid[x][y + 2];
        return
            grid[x + 1][y] + grid[x + 1][y + 1] + grid[x + 1][y + 2] == sum &&
            grid[x + 2][y] + grid[x + 2][y + 1] + grid[x + 2][y + 2] == sum &&
            grid[x][y] + grid[x + 1][y] + grid[x + 2][y] == sum &&
            grid[x][y + 1] + grid[x + 1][y + 1] + grid[x + 2][y + 1] == sum &&
            grid[x][y + 2] + grid[x + 1][y + 2] + grid[x + 2][y + 2] == sum &&
            grid[x][y] + grid[x + 1][y + 1] + grid[x + 2][y + 2] == sum &&
            grid[x][y + 2] + grid[x + 1][y + 1] + grid[x + 2][y] == sum;
    }
public:
    int numMagicSquaresInside(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();
        if (n < 3 || m < 3) return 0;

        int result = 0;
        for (int i = 0; i <= n - 3; ++i) {
            for (int j = 0; j <= m - 3; ++j) {
                if (isMagic(grid, i, j)) ++result;
            }
        }
        return result;
    }
};
