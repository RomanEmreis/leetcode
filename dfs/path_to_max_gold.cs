/*
  In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold in that cell, 0 if it is empty.
  Return the maximum amount of gold you can collect under the conditions:

  Every time you are located in a cell you will collect all the gold in that cell.
    * From your position, you can walk one step to the left, right, up, or down.
    * You can't visit the same cell more than once.
    * Never visit a cell with 0 gold.
  You can start and stop collecting gold from any position in the grid that has some gold.

  Example:
    Input: grid = [[0,6,0],[5,8,7],[0,9,0]]
    Output: 24
    Explanation:
      [[0,6,0],
       [5,8,7],
       [0,9,0]]
  Path to get the maximum gold, 9 -> 8 -> 7.
*/
public class Solution {
    public int GetMaximumGold(int[][] grid) {
        int result = 0;

        for (int i = 0; i < grid.Length; ++i) {
            for (int j = 0; j < grid[i].Length; ++j) {
                result = Math.Max(result, Backtrack(i, j, grid));
            }
        }

        return result;
    }

    private static int Backtrack(int i, int j, int[][] grid)
    {
        if (i < 0 || j < 0 || i >= grid.Length || j >= grid[i].Length) return 0;
        if (grid[i][j] == 0) return 0;

        int value = grid[i][j];
        int maximum = value;
        grid[i][j] = 0;

        maximum = Math.Max(maximum, value + Backtrack(i + 1, j, grid));
        maximum = Math.Max(maximum, value + Backtrack(i - 1, j, grid));
        maximum = Math.Max(maximum, value + Backtrack(i, j + 1, grid));
        maximum = Math.Max(maximum, value + Backtrack(i, j - 1, grid));

        grid[i][j] = value;

        return maximum;
    }
}
