/*
  You are given an m x n binary matrix grid.
  A move consists of choosing any row or column and toggling each value in that row or column (i.e., changing all 0's to 1's, and all 1's to 0's).

  Every row of the matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
  Return the highest possible score after making any number of moves (including zero moves).

  Example 1:
    Input: grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
    Output: 39
    Explanation: 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
*/
func matrixScore(grid [][]int) int {
    n, m := len(grid), len(grid[0]);
    result := n;

    for j := 1; j < m; j++ {
        set := 0;
        for i := 0; i < n; i++ {
            if grid[i][0] == grid[i][j] {
                set++;
            }
        }

        result = (result << 1) + int(math.Max(float64(set), float64(n - set)))
    }

    return result;
}
