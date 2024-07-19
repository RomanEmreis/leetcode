/*
  Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

  A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

  Example 1:
    Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
    Output: [15]
    Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.

  Example 2:
    Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
    Output: [12]
    Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
*/
public class Solution {
    public IList<int> LuckyNumbers (int[][] matrix) {
        Dictionary<int, int> rowMin = [];
        Dictionary<int, int> colMax = [];

        for (int i = 0; i < matrix.Length; ++i) {
            int min = matrix[i].Min();
            rowMin[min] = i;
        }

        for (int i = 0; i < matrix[0].Length; ++i) {
            int max = 0;
            for (int j = 0; j < matrix.Length; ++j) {
                max = Math.Max(max, matrix[j][i]);
            }
            colMax[max] = i;
        }

        List<int> result = [];

        foreach (var (min, row) in rowMin) {
            if (colMax.ContainsKey(min)) result.Add(min);
        }

        return result;
    }
}
