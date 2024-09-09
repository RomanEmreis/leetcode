/*
  You are given two integers m and n, which represent the dimensions of a matrix.
  You are also given the head of a linked list of integers.
  Generate an m x n matrix that contains the integers in the linked list presented in spiral order (clockwise), starting from the top-left of the matrix. If there are remaining empty spaces, fill them with -1.
  
  Return the generated matrix.
  
  Example 1:
    Input: m = 3, n = 5, head = [3,0,2,6,8,1,7,9,4,2,5,5,0]
    Output: [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]
    Explanation: The diagram above shows how the values are printed in the matrix.
    Note that the remaining spaces in the matrix are filled with -1.
  
  Example 2:
    Input: m = 1, n = 4, head = [0,1,2]
    Output: [[0,1,2,-1]]
    Explanation: The diagram above shows how the values are printed from left to right in the matrix.
    The last space in the matrix is set to -1.
*/
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func spiralMatrix(m int, n int, head *ListNode) [][]int {
    result := make([][]int, m);
    for i := 0; i < m; i++ {
        result[i] = make([]int, n);
        for j := range result[i] {
            result[i][j] = -1;
        }
    }

    rowBegin, rowEnd := 0, m - 1;
    colBegin, colEnd := 0, n - 1;

    for rowBegin <= rowEnd && colBegin <= colEnd {
        for j := colBegin; head != nil && j <= colEnd; j++ {
            result[rowBegin][j] = head.Val;
            head = head.Next;
        }

        rowBegin++;

        for i := rowBegin; head != nil && i <= rowEnd; i++ {
            result[i][colEnd] = head.Val;
            head = head.Next;
        }

        colEnd--;

        if rowBegin <= rowEnd {
            for j := colEnd; head != nil && j >= colBegin; j-- {
                result[rowEnd][j] = head.Val;
                head = head.Next;
            }
        }

        rowEnd--;

        if colBegin <= colEnd {
            for i := rowEnd; head != nil && i >= rowBegin; i-- {
                result[i][colBegin] = head.Val;
                head = head.Next;
            }
        }

        colBegin++;
    }

    return result;
}
