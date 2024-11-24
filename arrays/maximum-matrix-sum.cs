/*
  You are given an n x n integer matrix. You can do the following operation any number of times:
  
  Choose any two adjacent elements of matrix and multiply each of them by -1.
  Two elements are considered adjacent if and only if they share a border.
  
  Your goal is to maximize the summation of the matrix's elements. Return the maximum sum of the matrix's elements using the operation mentioned above.
  
  Example 1:
  Input: matrix = [[1,-1],[-1,1]]
  Output: 4
  Explanation: We can follow the following steps to reach sum equals 4:
  - Multiply the 2 elements in the first row by -1.
  - Multiply the 2 elements in the first column by -1.
*/
public class Solution {
    public long MaxMatrixSum(int[][] matrix) {
        long result = 0;
        int negativeCount = 0, min = int.MaxValue;
        for (int i = 0; i < matrix.Length; ++i) {
            for (int j = 0; j < matrix[i].Length; ++j) {
                int num = matrix[i][j];
                int absNum = Math.Abs(num);
                result += absNum;
                if (num < 0) ++negativeCount;
                min = Math.Min(min, absNum);
            }
        }

        if (negativeCount % 2 == 0) return result;
        else return result - 2 * min;
    }
}
