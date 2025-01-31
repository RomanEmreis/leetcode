/*
  827. Making A Large Island
  
  You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.
  Return the size of the largest island in grid after applying this operation.
  
  An island is a 4-directionally connected group of 1s.
  
  Example 1:
  Input: grid = [[1,0],[0,1]]
  Output: 3
  Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
  
  Example 2:
  Input: grid = [[1,1],[1,0]]
  Output: 4
  Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
  
  Example 3:
  Input: grid = [[1,1],[1,1]]
  Output: 4
  Explanation: Can't change any 0 to 1, only one island with area = 4.
*/
public class Solution {
    public int LargestIsland(int[][] grid) {
        int n = grid.Length;
        Span<int> directions = [-1, 0, 1, 0, -1];
        Span<int> parent = stackalloc int[n * n];
        Span<int> size = stackalloc int[n * n];
        
        for (int i = 0; i < size.Length; ++i) {
            size[i] = 1;
            parent[i] = i;
        }

        int result = 1;

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    for (int k = 0; k < 4; ++k) {
                        int x = i + directions[k];
                        int y = j + directions[k + 1];
                        if (x >= 0 && x < n && y >= 0 && y < n && grid[x][y] == 1) {
                            int a = Find(x * n + y, ref parent);
                            int b = Find(i * n + j, ref parent);
                            if (a != b) {
                                parent[a] = b;
                                size[b] += size[a];
                                if (result < size[b]) result = size[b];
                            }
                        }
                    }
                }
            }
        }

        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 0) {
                    int initialSize = 1;
                    HashSet<int> visited = [];
                    for (int k = 0; k < 4; ++k) {
                        int x = i + directions[k];
                        int y = j + directions[k + 1];
                        if (x >= 0 && x < n && y >= 0 && y < n && grid[x][y] == 1) {
                            int root = Find(x * n + y, ref parent);
                            if (visited.Add(root)) {
                                initialSize += size[root];
                            }
                        }
                    }
                    if (result < initialSize) result = initialSize;
                }
            }
        }

        return result;

        static int Find(int x, ref Span<int> parent) {
            if (x != parent[x]) {
                parent[x] = Find(parent[x], ref parent);
            }
            return parent[x];
        }
    }
}
