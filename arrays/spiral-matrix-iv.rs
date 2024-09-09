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
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        let mut result = vec![vec![-1; n]; m];

        let (mut row_begin, mut row_end) = (0 as usize, m - 1);
        let (mut col_begin, mut col_end) = (0 as usize, n - 1);

        while row_begin <= row_end && col_begin <= col_end {
            for j in col_begin..=col_end {
                if let Some(node) = head.take() {
                    result[row_begin][j] = node.val;
                    head = node.next;
                } else {
                    return result;
                }
            }

            row_begin += 1;

            for i in row_begin..=row_end {
                if let Some(node) = head.take() {
                    result[i][col_end] = node.val;
                    head = node.next;
                } else {
                    return result;
                }
            }

            col_end -= 1;

            if row_begin <= row_end {
                for j in (col_begin..=col_end).rev() {
                    if let Some(node) = head.take() {
                        result[row_end][j] = node.val;
                        head = node.next;
                    } else {
                        return result;
                    }
                }
            }

            row_end -= 1;

            if col_begin <= col_end {
                for i in (row_begin..=row_end).rev() {
                    if let Some(node) = head.take() {
                        result[i][col_begin] = node.val;
                        head = node.next;
                    } else {
                        return result;
                    }
                }
            }

            col_begin += 1;
        }

        result
    }
}
