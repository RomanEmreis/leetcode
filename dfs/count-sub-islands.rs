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
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid2.len();
        let n = grid2[0].len();
        let mut result = 0;
        let mut grid2 = grid2;

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 && Self::dfs(&grid1, &mut grid2, i, j) {
                    result += 1;
                }
            }
        }

        result
    }

    fn dfs(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        let m = grid2.len();
        let n = grid2[0].len();

        if i >= m || j >= n {
            return true;
        }

        if grid2[i][j] == 0 {
            return true;
        }

        if grid1[i][j] == 0 {
            return false;
        }

        grid2[i][j] = 0;

        let up = if i > 0 { Self::dfs(grid1, grid2, i - 1, j) } else { true };
        let down = if i + 1 < m { Self::dfs(grid1, grid2, i + 1, j) } else { true };
        let left = if j > 0 { Self::dfs(grid1, grid2, i, j - 1) } else { true };
        let right = if j + 1 < n { Self::dfs(grid1, grid2, i, j + 1) } else { true };

        up && down && left && right
    }
}
