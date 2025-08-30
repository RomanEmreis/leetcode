/*
  36. Valid Sudoku
  
  Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
      Each row must contain the digits 1-9 without repetition.
      Each column must contain the digits 1-9 without repetition.
      Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
  
  Note:
      A Sudoku board (partially filled) could be valid but is not necessarily solvable.
      Only the filled cells need to be validated according to the mentioned rules.
  
  Example 1:
  Input: board = 
  [["5","3",".",".","7",".",".",".","."]
  ,["6",".",".","1","9","5",".",".","."]
  ,[".","9","8",".",".",".",".","6","."]
  ,["8",".",".",".","6",".",".",".","3"]
  ,["4",".",".","8",".","3",".",".","1"]
  ,["7",".",".",".","2",".",".",".","6"]
  ,[".","6",".",".",".",".","2","8","."]
  ,[".",".",".","4","1","9",".",".","5"]
  ,[".",".",".",".","8",".",".","7","9"]]
  Output: true
  
  Example 2:
  Input: board = 
  [["8","3",".",".","7",".",".",".","."]
  ,["6",".",".","1","9","5",".",".","."]
  ,[".","9","8",".",".",".",".","6","."]
  ,["8",".",".",".","6",".",".",".","3"]
  ,["4",".",".","8",".","3",".",".","1"]
  ,["7",".",".",".","2",".",".",".","6"]
  ,[".","6",".",".",".",".","2","8","."]
  ,[".",".",".","4","1","9",".",".","5"]
  ,[".",".",".",".","8",".",".","7","9"]]
  Output: false
  Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
*/
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::with_capacity(9);
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' && !set.insert(board[i][j]) {
                    return false;
                }
            }
            set.clear();
            for j in 0..9 {
                if board[j][i] != '.' && !set.insert(board[j][i]) {
                    return false;
                }
            }
            set.clear();
        }
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                for k in 0..3 {
                    if board[i][j + k] != '.' && !set.insert(board[i][j + k]) {
                        return false;
                    }
                    if board[i + 1][j + k] != '.' && !set.insert(board[i + 1][j + k]) {
                        return false;
                    }
                    if board[i + 2][j + k] != '.' && !set.insert(board[i + 2][j + k]) {
                        return false;
                    }
                }
                set.clear();
            }
        }
        true
    }
}
