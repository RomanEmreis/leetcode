/*
  You are given an m x n binary matrix matrix.
  You can choose any number of columns in the matrix and flip every cell in that column (i.e., Change the value of the cell from 0 to 1 or vice versa).
  
  Return the maximum number of rows that have all values equal after some number of flips.
  
  Example 1:
  Input: matrix = [[0,1],[1,1]]
  Output: 1
  Explanation: After flipping no values, 1 row has all values equal.
  
  Example 2:
  Input: matrix = [[0,1],[1,0]]
  Output: 2
  Explanation: After flipping values in the first column, both rows have equal values.

  Example 3:
  Input: matrix = [[0,0,0],[0,0,1],[1,1,0]]
  Output: 2
  Explanation: After flipping values in the first two columns, the last two rows have equal values.
*/
public class Solution {
    public int MaxEqualRowsAfterFlips(int[][] matrix) {
        int result = 0;
        int m = matrix.Length, n = matrix[0].Length;
        Span<int> flip = stackalloc int[n];

        for (int i = 0; i < matrix.Length; ++i) {
            int count = 0;
            for (int j = 0; j < matrix[i].Length; ++j) {
                flip[j] = 1 - matrix[i][j];
            }
            for (int j = 0; j < matrix.Length; ++j) {
                if (
                    matrix[i].AsSpan().SequenceEqual(matrix[j]) || 
                    matrix[j].AsSpan().SequenceEqual(flip)
                ) ++count;
            }

            if (result < count) result = count;
        }

        return result;
    }
}
