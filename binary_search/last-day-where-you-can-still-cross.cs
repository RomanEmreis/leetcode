/*
  1970. Last Day Where You Can Still Cross
  
  There is a 1-based binary matrix where 0 represents land and 1 represents water. 
  You are given integers row and col representing the number of rows and columns in the matrix, respectively.
  Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. 
  You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the ith day, 
  the cell on the rith row and cith column (1-based coordinates) will be covered with water (i.e., changed to 1).
  You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. 
  You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).
  
  Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.
  
  Example 1:
  Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
  Output: 2
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 2.
  
  Example 2:
  Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
  Output: 1
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 1.
  
  Example 3:
  Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
  Output: 3
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 3.
*/
public class Solution {
    public int LatestDayToCross(int row, int col, int[][] cells) {
        int l = 1;
        int r = cells.Length - 1;

        int res = 0;

        while (l <= r) {
            int m = (l + r) >> 1;
            if (CanCross(m, row, col, cells)) {
                l = m + 1;
                res = m;
            } else {
                r = m - 1;
            }
        }

        return res;

        static bool CanCross(int day, int row, int col, int[][] cells) {
            Span<(int, int)> dirs = [
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0)
            ];

            int[,] matrix = new int[row, col];
            for (int i = 0; i < day; ++i) {
                int x = cells[i][0] - 1;
                int y = cells[i][1] - 1;
                matrix[x, y] = 1;
            }

            Queue<(int, int)> q = [];
            for (int i = 0; i < col; ++i)
                if (matrix[0, i] == 0) {
                    q.Enqueue((0, i));
                    matrix[0, i] = 1;
                }

            while (q.TryDequeue(out var item)) {
                var (i, j) = item;
                foreach (var (dx, dy) in dirs) {
                    int x = i + dx;
                    int y = j + dy;

                    if (x == row || y == col || x < 0 || y < 0)
                        continue;

                    if (matrix[x, y] == 1)
                        continue;

                    if (x == row - 1)
                        return true;

                    q.Enqueue((x, y));
                    matrix[x, y] = 1;
                }
            }

            return false;
        }
    }
}
