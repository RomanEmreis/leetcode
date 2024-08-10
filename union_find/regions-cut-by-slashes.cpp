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
struct union_find {
    int count;
    vector<int> parent;
    union_find(int n);
    int find(int x);
    void union_(int x, int y);
};

union_find::union_find(int n) : parent(n), count(n) {
    for (size_t i = 0; i < n; ++i) {
        parent[i] = i;
    }
}

int union_find::find(int x) {
    if (parent[x] != x) parent[x] = find(parent[x]);
    return parent[x];
}

void union_find::union_(int x, int y) {
    int root_x = find(x);
    int root_y = find(y);
    if (root_x != root_y) {
        parent[root_y] = root_x;
        --count;
    }
}

class Solution {
public:
    int regionsBySlashes(vector<string>& grid) {
        int n = grid.size();
        int size = n * n * 4;

        union_find uf(size);

        for (size_t i = 0; i < n; ++i) {
            string& row = grid[i];
            for (size_t j = 0; j < n; ++j) {
                int root = 4 * (i * n + j);
                char& ch = row[j];

                if (ch != '\\') {
                    uf.union_(root, root + 1);
                    uf.union_(root + 2, root + 3);
                }
                if (ch != '/') {
                    uf.union_(root, root + 3);
                    uf.union_(root + 1, root + 2);
                }

                if (i + 1 < n) uf.union_(root + 3, root + 4 * n + 1);
                if (j + 1 < n) uf.union_(root + 2, root + 4);
            }
        }

        return uf.count;
    }
};
