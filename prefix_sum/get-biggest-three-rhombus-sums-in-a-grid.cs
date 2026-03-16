/*
  1878. Get Biggest Three Rhombus Sums in a Grid
  
  You are given an m x n integer matrix grid​​​.
  A rhombus sum is the sum of the elements that form the border of a regular rhombus shape in grid​​​. The rhombus must have the shape of a square rotated 45 degrees 
  with each of the corners centered in a grid cell. Below is an image of four valid rhombus shapes with the corresponding colored cells that should be included in each rhombus sum:
  
  Note that the rhombus can have an area of 0, which is depicted by the purple rhombus in the bottom right corner.
  
  Return the biggest three distinct rhombus sums in the grid in descending order. If there are less than three distinct values, return all of them.
  
  Example 1:
  Input: grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]]
  Output: [228,216,211]
  Explanation: The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
  - Blue: 20 + 3 + 200 + 5 = 228
  - Red: 200 + 2 + 10 + 4 = 216
  - Green: 5 + 200 + 4 + 2 = 211
  
  Example 2:
  Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
  Output: [20,9,8]
  Explanation: The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
  - Blue: 4 + 2 + 6 + 8 = 20
  - Red: 9 (area 0 rhombus in the bottom right corner)
  - Green: 8 (area 0 rhombus in the bottom middle)
  
  Example 3:
  Input: grid = [[7,7,7]]
  Output: [7]
  Explanation: All three possible rhombus sums are the same, so return [7].
*/
public class Solution {
    public int[] GetBiggestThree(int[][] grid) {
        int rows = grid.Length; 
        int cols = grid[0].Length;
        int f = 0, s = 0, t = 0;

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                Update(ref f, ref s, ref t, grid[r][c]);

                int maxK = Math.Min(Math.Min(c, cols - 1 - c), (rows - 1 - r) / 2);

                for (int k = 1; k <= maxK; k++) {
                    int sum = grid[r][c] + grid[r + 2 * k][c] + grid[r + k][c - k] + grid[r + k][c + k];

                    for (int i = 1; i < k; i++) {
                        sum += grid[r + i][c + i];
                        sum += grid[r + i][c - i];
                        sum += grid[r + k + i][c - k + i];
                        sum += grid[r + k + i][c + k - i];
                    }
                    
                    Update(ref f, ref s, ref t, sum);
                }
            }
        }

        int count = (f > 0 ? 1 : 0) + (s > 0 ? 1 : 0) + (t > 0 ? 1 : 0);
        int[] res = new int[count];

        if (f > 0) 
            res[0] = f;

        if (s > 0 && count > 1) 
            res[1] = s;

        if (t > 0 && count > 2) 
            res[2] = t;
        
        return res;
    }

    private static void Update(ref int f, ref int s, ref int t, int v) {
        if (v == f || v == s || v == t)
            return;
        if (v > f) { 
            t = s; 
            s = f; 
            f = v;
        } else if (v > s) {
            t = s; 
            s = v;
        } else if (v > t) 
            t = v;
    }
}
