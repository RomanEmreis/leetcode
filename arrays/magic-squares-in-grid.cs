/*
  A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9 such that each row, column, and both diagonals all have the same sum.
  Given a row x col grid of integers, how many 3 x 3 contiguous magic square subgrids are there?
  Note: while a magic square can only contain numbers from 1 to 9, grid may contain numbers up to 15.
*/
public class Solution {
    public int NumMagicSquaresInside(int[][] grid) {
        if (grid.Length < 3) return 0;
        if (grid[0].Length < 3) return 0;

        int result = 0;

        for (int i = 0; i <= grid.Length - 3; ++i) {
            for (int j = 0; j <= grid[0].Length - 3; ++j) {
                if (IsMagic(i, j, grid)) ++result;
            }
        }

        return result;
    }

    private static bool IsMagic(int x, int y, int[][] grid) {
        HashSet<int> uniqueNumbers = [];
        
        for (int i = x; i < x + 3; i++) {
            for (int j = y; j < y + 3; j++) {
                int num = grid[i][j];
                if (num < 1 || num > 9 || !uniqueNumbers.Add(num)) return false;
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
}
