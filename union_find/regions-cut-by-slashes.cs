/*
  An n x n grid is composed of 1 x 1 squares where each 1 x 1 square consists of a '/', '\', or blank space ' '. These characters divide the square into contiguous regions.
  Given the grid grid represented as a string array, return the number of regions.

  Note that backslash characters are escaped, so a '\' is represented as '\\'.

  Example 1:
    Input: grid = [" /","/ "]
    Output: 2
  
  Example 2:
    Input: grid = [" /","  "]
    Output: 1

  Example 3:
    Input: grid = ["/\\","\\/"]
    Output: 5
    Explanation: Recall that because \ characters are escaped, "\\/" refers to \/, and "/\\" refers to /\.
*/
public class Solution {
    public int RegionsBySlashes(string[] grid) {
        int n = grid.Length;
        int size = 4 * n * n;
        
        UnionFind uf = new(size);

        for (int i = 0; i < n; ++i) {
            ReadOnlySpan<char> row = grid[i].AsSpan();
            for (int j = 0; j < n; ++j) {
                int root = 4 * (i * n + j);
                char ch = row[j];

                if (ch != '\\') {
                    uf.Union(root + 0, root + 1);
                    uf.Union(root + 2, root + 3);
                }
                if (ch != '/') {
                    uf.Union(root + 0, root + 3);
                    uf.Union(root + 1, root + 2);
                }

                if (i + 1 < n) uf.Union(root + 3, root + 4 * n + 1);
                if (i - 1 >= 0) uf.Union(root + 1, root - 4 * n + 3);
                if (j + 1 < n) uf.Union(root + 2, root + 4);
                if (j - 1 >= 0) uf.Union(root, root - 4 + 2);
            }
        }

        return uf.count;
    }
}

public ref struct UnionFind {
    private readonly Span<int> parent;
    public int count;
    public UnionFind(int n) {
        parent = new int[n];
        count = n;
        for (int i = 0; i < n; ++i) parent[i] = i;
    }

    public int Find(int x) {
        if (parent[x] != x) parent[x] = Find(parent[x]);
        return parent[x];
    }

    public void Union(int x, int y) {
        int rootX = Find(x);
        int rootY = Find(y);
        if (rootX != rootY) {
            parent[rootY] = rootX;
            --count;
        }
    }
}
