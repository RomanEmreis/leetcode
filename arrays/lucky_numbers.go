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
func luckyNumbers (matrix [][]int) []int {
    rowMin, colMax := make(map[int]int), make(map[int]int);
    result := make([]int, 0);

    const maxInt = int(^uint(0) >> 1);

    for i := 0; i < len(matrix); i++ {
        min := maxInt;
        for j := 0; j < len(matrix[i]); j++ {
            if min > matrix[i][j] {
                min = matrix[i][j];
            }
        }
        rowMin[min] = i;
    }

    for i := 0; i < len(matrix[0]); i++ {
        max := 0;
        for j := 0; j < len(matrix); j++ {
            if max < matrix[j][i] {
                max = matrix[j][i];
            }
        }
        colMax[max] = i;
    }

    for min, _ := range rowMin {
        if _, ok := colMax[min]; ok {
            result = append(result, min);
        }
    }

    return result;
}
