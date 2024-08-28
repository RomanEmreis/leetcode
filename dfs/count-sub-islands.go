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
func countSubIslands(grid1 [][]int, grid2 [][]int) int {
    m, n := len(grid2), len(grid2[0]);
    result := 0;

    for i := 0; i < m; i++ {
        for j := 0; j < n; j++ {
            if grid2[i][j] == 1 {
                if dfs(&grid1, &grid2, i, j) {
                    result++;
                }
            }
        }
    }

    return result;
}

func dfs(grid1 *[][]int, grid2 *[][]int, i, j int) bool {
    m, n := len(*grid2), len((*grid2)[0]);
    if i < 0 || j < 0 || i >= m || j >= n {
        return true;
    }
    if (*grid2)[i][j] == 0 {
        return true;
    }
    if (*grid1)[i][j] == 0 {
        return false;
    }

    (*grid2)[i][j] = 0;

	up := dfs(grid1, grid2, i - 1, j);
	down := dfs(grid1, grid2, i + 1, j);
	left := dfs(grid1, grid2, i, j - 1);
	right := dfs(grid1, grid2, i, j + 1);

	return up && down && left && right;
}
