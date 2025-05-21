/*
  73. Set Matrix Zeroes
  
  Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
  You must do it in place.
  
  Example 1:
  Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
  Output: [[1,0,1],[0,0,0],[1,0,1]]
  
  Example 2:
  Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
  Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
*/
public class Solution {
    public void SetZeroes(int[][] matrix) {
        bool i0 = false;
        bool j0 = false;

        for (int i = 0; i < matrix.Length; ++i) {
            if (matrix[i][0] == 0) {
                j0 = true;
                break;
            }
        }

        for (int j = 0; j < matrix[0].Length; ++j) {
            if (matrix[0][j] == 0) {
                i0 = true;
            }
        }

        for (int i = 1; i < matrix.Length; ++i) {
            for (int j = 1; j < matrix[0].Length; ++j) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for (int i = 1; i < matrix.Length; ++i) {
            for (int j = 1; j < matrix[0].Length; ++j) {
                if (matrix[i][0] == 0 || matrix[0][j] == 0) {
                    matrix[i][j] = 0;
                }
            }
        }

        if (i0) {
            for (int j = 0; j < matrix[0].Length; ++j) {
                matrix[0][j] = 0;
            }
        }

        if (j0) {
            for (int i = 0; i < matrix.Length; ++i) {
                matrix[i][0] = 0;
            }
        }
    }
}
