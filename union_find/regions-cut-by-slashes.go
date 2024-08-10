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
type UnionFind struct {
    count int;
	parent []int;
}

func NewUnionFind(n int) *UnionFind {
	count := n;
    parent := make([]int, n);
	for i := range parent {
		parent[i] = i;
	}
	return &UnionFind{count, parent};
}

func (uf *UnionFind) Find(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.Find(uf.parent[x]);
	}
	return uf.parent[x];
}

func (uf *UnionFind) Union(x, y int) {
	rootX := uf.Find(x);
	rootY := uf.Find(y);
	if rootX != rootY {
		uf.parent[rootY] = rootX;
        uf.count--;
	}
}

func regionsBySlashes(grid []string) int {
    n := len(grid)
	size := 4 * n * n
	uf := NewUnionFind(size)

	for r := 0; r < n; r++ {
		for c := 0; c < n; c++ {
			root := 4 * (r*n + c)
			val := grid[r][c]

			if val != '\\' {
				uf.Union(root+0, root+1)
				uf.Union(root+2, root+3)
			}
			if val != '/' {
				uf.Union(root+0, root+3)
				uf.Union(root+1, root+2)
			}

			if r+1 < n {
				uf.Union(root+3, root+4*n+1)
			}
			if c+1 < n {
				uf.Union(root+2, root+4+0)
			}
		}
	}

    return uf.count;
}
