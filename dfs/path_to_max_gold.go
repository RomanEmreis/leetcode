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
func getMaximumGold(grid [][]int) int {
    n, m := len(grid), len(grid[0]);
    result := 0;

    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            result = int(math.Max(float64(result), float64(backtrack(i, j, &n, &m, &grid))))
        }
    }

    return result;
}

func backtrack(i, j int, n, m *int, pGrid *[][]int) int {
    grid := *pGrid;
    if i < 0 || j < 0 || i >= *n || j >= *m || grid[i][j] == 0 {
        return 0;
    }

    val := grid[i][j];
    max := val;
    grid[i][j] = 0;

    max = int(math.Max(float64(max), float64(val + backtrack(i - 1, j, n, m, pGrid))));
    max = int(math.Max(float64(max), float64(val + backtrack(i + 1, j, n, m, pGrid))));
    max = int(math.Max(float64(max), float64(val + backtrack(i, j + 1, n, m, pGrid))));
    max = int(math.Max(float64(max), float64(val + backtrack(i, j - 1, n, m, pGrid))));

    grid[i][j] = val;

    return max;
}
