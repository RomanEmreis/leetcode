/*
  2267. Check if There Is a Valid Parentheses String Path
  
  A parentheses string is a non-empty string consisting only of '(' and ')'. It is valid if any of the following conditions is true:
      It is ().
      It can be written as AB (A concatenated with B), where A and B are valid parentheses strings.
      It can be written as (A), where A is a valid parentheses string.
  
  You are given an m x n matrix of parentheses grid. A valid parentheses string path in the grid is a path satisfying all of the following conditions:
      The path starts from the upper left cell (0, 0).
      The path ends at the bottom-right cell (m - 1, n - 1).
      The path only ever moves down or right.
      The resulting parentheses string formed by the path is valid.
  
  Return true if there exists a valid parentheses string path in the grid. Otherwise, return false.
  
  Example 1:
  Input: grid = [["(","(","("],[")","(",")"],["(","(",")"],["(","(",")"]]
  Output: true
  Explanation: The above diagram shows two possible paths that form valid parentheses strings.
  The first path shown results in the valid parentheses string "()(())".
  The second path shown results in the valid parentheses string "((()))".
  Note that there may be other valid parentheses string paths.
  
  Example 2:
  Input: grid = [[")",")"],["(","("]]
  Output: false
  Explanation: The two possible paths form the parentheses strings "))(" and ")((". Since neither of them are valid parentheses strings, we return false.
*/
impl Solution {
    #[inline]
    fn add_open(bits: [u64; 4]) -> [u64; 4] {
        [
            bits[0] << 1,
            (bits[1] << 1) | (bits[0] >> 63),
            (bits[2] << 1) | (bits[1] >> 63),
            (bits[3] << 1) | (bits[2] >> 63),
        ]
    }

    #[inline]
    fn add_close(bits: [u64; 4]) -> [u64; 4] {
        [
            (bits[0] >> 1) | (bits[1] << 63),
            (bits[1] >> 1) | (bits[2] << 63),
            (bits[2] >> 1) | (bits[3] << 63),
            bits[3] >> 1,
        ]
    }

    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let path_len = rows + cols - 1;

        if path_len % 2 == 1
            || grid[0][0] != '('
            || grid[rows - 1][cols - 1] != ')'
        {
            return false;
        }

        let mut previous = vec![[0u64; 4]; cols];
        let mut current = vec![[0u64; 4]; cols];

        for row in 0..rows {
            for col in 0..cols {
                let mut incoming = [0u64; 4];

                if row == 0 && col == 0 {
                    incoming[0] = 1;
                } else {
                    for word in 0..4 {
                        let from_top = if row > 0 {
                            previous[col][word]
                        } else {
                            0
                        };

                        let from_left = if col > 0 {
                            current[col - 1][word]
                        } else {
                            0
                        };

                        incoming[word] = from_top | from_left;
                    }
                }

                current[col] = if grid[row][col] == '(' {
                    Self::add_open(incoming)
                } else {
                    Self::add_close(incoming)
                };
            }

            std::mem::swap(&mut previous, &mut current);
        }

        previous[cols - 1][0] & 1 != 0
    }
}
