/*
  You are given two integers m and n representing a 0-indexed m x n grid. You are also given two 2D integer arrays guards and walls where guards[i] = [rowi, coli] and walls[j] = [rowj, colj] represent the positions of the ith guard and jth wall respectively.
  A guard can see every cell in the four cardinal directions (north, east, south, or west) starting from their position unless obstructed by a wall or another guard. A cell is guarded if there is at least one guard that can see it.
  
  Return the number of unoccupied cells that are not guarded.
  
  Example 1:
  Input: m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
  Output: 7
  Explanation: The guarded and unguarded cells are shown in red and green respectively in the above diagram.
  There are a total of 7 unguarded cells, so we return 7.
*/
public class Solution {
    public int CountUnguarded(int m, int n, int[][] guards, int[][] walls) {
        int[,] matrix = new int[m,n];
        for (int i = 0; i < guards.Length; ++i) {
            matrix[guards[i][0], guards[i][1]] = 2;
        }
        for (int i = 0; i < walls.Length; ++i) {
            matrix[walls[i][0], walls[i][1]] = 2;
        }

        Span<int> directions = [-1, 0, 1, 0, -1];
        foreach (int[] guard in guards) {
            for (int k = 0; k < 4; ++k) {
                int x = guard[0];
                int y = guard[1];
                int a = directions[k];
                int b = directions[k + 1];
                while (x + a >= 0 && x + a < m && y + b >= 0 && y + b < n && matrix[x + a, y + b] < 2) {
                    x += a;
                    y += b;
                    matrix[x, y] = 1;
                }
            }
        }

        int result = 0;
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (matrix[i, j] == 0) ++result;
            }
        }

        return result;
    }
}
