/*
  Given a m * n matrix of ones and zeros, return how many square submatrices have all ones.
  
  Example 1:
    Input: matrix =
    [
      [0,1,1,1],
      [1,1,1,1],
      [0,1,1,1]
    ]
    Output: 15
    Explanation: 
    There are 10 squares of side 1.
    There are 4 squares of side 2.
    There is  1 square of side 3.
    Total number of squares = 10 + 4 + 1 = 15.
  
  Example 2:
    Input: matrix = 
    [
      [1,0,1],
      [1,1,0],
      [1,1,0]
    ]
    Output: 7
    Explanation: 
    There are 6 squares of side 1.  
    There is 1 square of side 2. 
    Total number of squares = 6 + 1 = 7.
*/
public class Solution {
    public int CountSquares(int[][] matrix) {
        int m = matrix.Length;
        int n = matrix[0].Length;

        int result = 0;

        for (int i = 0; i < m; ++i) {
            result += matrix[i][0];
        }

        for (int i = 1; i < n; ++i) {
            result += matrix[0][i];
        }

        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                if (matrix[i][j] == 0) continue;

                int a = matrix[i][j - 1];
                int b = matrix[i - 1][j - 1];
                int c = matrix[i - 1][j];

                if (a != 0 && b != 0 && c != 0) {
                    int min = Math.Min(a, Math.Min(b, c));
                    matrix[i][j] = min + 1;
                }

                result += matrix[i][j];
            }
        }

        return result;
    }
}
