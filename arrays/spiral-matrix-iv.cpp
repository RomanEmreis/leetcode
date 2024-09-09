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
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    vector<vector<int>> spiralMatrix(int m, int n, ListNode* head) {
        vector<vector<int>> result(m, vector<int>(n, -1));

        int row_begin = 0, row_end = m - 1;
        int col_begin = 0, col_end = n - 1;

        while (row_begin <= row_end && col_begin <= col_end) {
            for (int j = col_begin; j <= col_end; ++j) {
                if (head) {
                    result[row_begin][j] = head->val;
                    head = head->next;
                } else {
                    return result;
                }
            }

            ++row_begin;

            for (int i = row_begin; i <= row_end; ++i) {
                if (head) {
                    result[i][col_end] = head->val;
                    head = head->next;
                } else {
                    return result;
                }
            }

            --col_end;

            if (row_begin <= row_end) {
                for (int j = col_end; j >= col_begin; --j) {
                    if (head) {
                        result[row_end][j] = head->val;
                        head = head->next;
                    } else {
                        return result;
                    }
                }
            }

            --row_end;

            if (col_begin <= col_end) {
                for (int i = row_end; i >= row_begin; --i) {
                    if (head) {
                        result[i][col_begin] = head->val;
                        head = head->next;
                    } else {
                        return result;
                    }
                }
            }

            ++col_begin;
        }

        return result;
    }
};
