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
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */
public class Solution {
    public int[][] SpiralMatrix(int m, int n, ListNode head) {
        int[][] result = new int[m][];

        for (int k = 0; k < m; ++k) {
            result[k] = new int[n];
            Array.Fill(result[k], -1);
        }

        int rowBegin = 0, colBegin = 0;
        int rowEnd = m - 1, colEnd = n - 1;

        while (rowBegin <= rowEnd && colBegin <= colEnd) {
            for (int j = colBegin; head != null && j <= colEnd; ++j) {
                result[rowBegin][j] = head.val;
                head = head.next;
            }

            ++rowBegin;

            for (int i = rowBegin; head != null && i <= rowEnd; ++i) {
                result[i][colEnd] = head.val;
                head = head.next;
            }

            --colEnd;

            if (rowBegin <= rowEnd) {
                for (int j = colEnd; head != null && j >= colBegin; --j) {
                    result[rowEnd][j] = head.val;
                    head = head.next;
                }
            }
            --rowEnd;

            if (colBegin <= colEnd) {
                for (int i = rowEnd; head != null && i >= rowBegin; --i) {
                    result[i][colBegin] = head.val;
                    head = head.next;
                }
            }
            ++colBegin;
        }

        return result;
    }
}
