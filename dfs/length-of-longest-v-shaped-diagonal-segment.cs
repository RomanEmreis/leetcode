/*
  3459. Length of Longest V-Shaped Diagonal Segment
  
  You are given a 2D integer matrix grid of size n x m, where each element is either 0, 1, or 2.
  
  A V-shaped diagonal segment is defined as:
    The segment starts with 1.
    The subsequent elements follow this infinite sequence: 2, 0, 2, 0, ....
    The segment:
    Starts along a diagonal direction (top-left to bottom-right, bottom-right to top-left, top-right to bottom-left, or bottom-left to top-right).
    Continues the sequence in the same diagonal direction.
    Makes at most one clockwise 90-degree turn to another diagonal direction while maintaining the sequence.
  
  Return the length of the longest V-shaped diagonal segment. If no valid segment exists, return 0.
  
  Example 1:
  Input: grid = [[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
  Output: 5
  Explanation:
  The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: (0,2) → (1,3) → (2,4), takes a 90-degree clockwise turn at (2,4), and continues as (3,3) → (4,2).
  
  Example 2:
  Input: grid = [[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]
  Output: 4
  Explanation:
  The longest V-shaped diagonal segment has a length of 4 and follows these coordinates: (2,3) → (3,2), takes a 90-degree clockwise turn at (3,2), and continues as (2,1) → (1,0).
  
  Example 3:
  Input: grid = [[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]]
  Output: 5
  Explanation:
  The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: (0,0) → (1,1) → (2,2) → (3,3) → (4,4).
  
  Example 4:
  Input: grid = [[1]]
  Output: 1
  Explanation:
  The longest V-shaped diagonal segment has a length of 1 and follows these coordinates: (0,0).
*/
public class Solution {
    private static readonly int[,] kDirs = new int[4, 2] {
        { -1,  1 },
        {  1,  1 },
        {  1, -1 },
        { -1, -1 }
    };

    public int LenOfVDiagonal(int[][] grid) {
        int m = grid. Length;
        int n = grid[0].Length;

        // mem[i, j, turned, hashNum, dir]
        int[,,,,] mem = new int[m, n, 2, 2, 4];
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                for (int t = 0; t < 2; t++)
                    for (int h = 0; h < 2; h++)
                        for (int d = 0; d < 4; d++)
                            mem[i, j, t, h, d] = -1;

        int res = 0;

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    for (int d = 0; d < 4; ++d) {
                        int dx = kDirs[d, 0];
                        int dy = kDirs[d, 1];
                        res = Math.Max(res, 1 + Dfs(
                            grid, 
                            i + dx, 
                            j + dy, 
                            false, 
                            2, 
                            d, 
                            mem)
                        );
                    }
                }
            }
        }

        return res;
    }

    private int Dfs(int[][] grid, int i, int j, bool turned, int num, int dir, int[,,,,] mem) {
        int m = grid.Length;
        int n = grid[0].Length;

        if (i < 0 || i == m || j < 0 || j == n) return 0;
        if (grid[i][j] != num) return 0;

        int hashNum = Math.Max(0, num - 1);
        int t = turned ? 1 : 0;

        if (mem[i, j, t, hashNum, dir] != -1) return mem[i, j, t, hashNum, dir];

        int nextNum = num == 2 ? 0 : 2;
        int dx = kDirs[dir, 0];
        int dy = kDirs[dir, 1];

        int res = 1 + Dfs(
            grid, 
            i + dx, 
            j + dy, 
            turned, 
            nextNum, 
            dir, 
            mem
        );

        if (!turned) {
            int nextDir = (dir + 1) % 4;
            int nextDx = kDirs[nextDir, 0];
            int nextDy = kDirs[nextDir, 1];
            res = Math.Max(res, 1 + Dfs(
                grid, 
                i + nextDx, 
                j + nextDy,
                true, 
                nextNum, 
                nextDir, 
                mem
            ));
        }

        mem[i, j, t, hashNum, dir] = res;
        return res;
    }
}
