/*
  You are given an m x n integer matrix points (0-indexed). Starting with 0 points, you want to maximize the number of points you can get from the matrix.
  To gain points, you must pick one cell in each row. Picking the cell at coordinates (r, c) will add points[r][c] to your score.
  However, you will lose points if you pick a cell too far from the cell that you picked in the previous row. For every two adjacent rows r and r + 1 (where 0 <= r < m - 1), 
  picking cells at coordinates (r, c1) and (r + 1, c2) will subtract abs(c1 - c2) from your score.

  Return the maximum number of points you can achieve.

  abs(x) is defined as:
    x for x >= 0.
    -x for x < 0.

  Example 1:
    Input: points = [[1,2,3],[1,5,1],[3,1,1]]
    Output: 9
    Explanation:
    The blue cells denote the optimal cells to pick, which have coordinates (0, 2), (1, 1), and (2, 0).
    You add 3 + 5 + 3 = 11 to your score.
    However, you must subtract abs(2 - 1) + abs(1 - 0) = 2 from your score.
    Your final score is 11 - 2 = 9.
*/
func maxPoints(points [][]int) int64 {
    m := len(points)
    n := len(points[0])

    prevRow := make([]int64, n)
    for j := 0; j < n; j++ {
        prevRow[j] = int64(points[0][j])
    }
    
    for i := 1; i < m; i++ {
        currRow := make([]int64, n)
        leftMax := make([]int64, n)
        rightMax := make([]int64, n)

        leftMax[0] = prevRow[0]
        for j := 1; j < n; j++ {
            leftMax[j] = int64(math.Max(float64(leftMax[j-1]), float64(prevRow[j] + int64(j))))
        }

        rightMax[n-1] = prevRow[n-1] - int64(n-1)
        for j := n-2; j >= 0; j-- {
            rightMax[j] = int64(math.Max(float64(rightMax[j+1]), float64(prevRow[j] - int64(j))))
        }

        for j := 0; j < n; j++ {
            addFromLeft := leftMax[j] - int64(j)
            addFromRight := rightMax[j] + int64(j)
            currRow[j] = int64(points[i][j]) + int64(math.Max(float64(addFromLeft), float64(addFromRight)))
        }

        prevRow = currRow
    }
    
    maxPoints := prevRow[0]
    for j := 1; j < n; j++ {
        maxPoints = int64(math.Max(float64(maxPoints), float64(prevRow[j])))
    }

    return maxPoints
}
