/*
  1254. Number of Closed Islands
  
  Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally 
  connected group of 0s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
  
  Return the number of closed islands.
  
  Example 1:
  Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
  Output: 2
  Explanation: 
  Islands in gray are closed because they are completely surrounded by water (group of 1s).
  
  Example 2:
  Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
  Output: 1
  
  Example 3:
  Input: grid = [[1,1,1,1,1,1,1],
                 [1,0,0,0,0,0,1],
                 [1,0,1,1,1,0,1],
                 [1,0,1,0,1,0,1],
                 [1,0,1,1,1,0,1],
                 [1,0,0,0,0,0,1],
                 [1,1,1,1,1,1,1]]
  Output: 2
*/
impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn flood(
            grid: &mut [Vec<i32>],
            start_row: usize,
            start_col: usize,
            stack: &mut Vec<usize>,
        ) {
            if grid[start_row][start_col] != 0 {
                return;
            }

            let rows = grid.len();
            let cols = grid[0].len();

            grid[start_row][start_col] = 1;
            stack.push(start_row * cols + start_col);

            while let Some(position) = stack.pop() {
                let row = position / cols;
                let col = position % cols;

                if row > 0 && grid[row - 1][col] == 0 {
                    grid[row - 1][col] = 1;
                    stack.push((row - 1) * cols + col);
                }

                if row + 1 < rows && grid[row + 1][col] == 0 {
                    grid[row + 1][col] = 1;
                    stack.push((row + 1) * cols + col);
                }

                if col > 0 && grid[row][col - 1] == 0 {
                    grid[row][col - 1] = 1;
                    stack.push(row * cols + col - 1);
                }

                if col + 1 < cols && grid[row][col + 1] == 0 {
                    grid[row][col + 1] = 1;
                    stack.push(row * cols + col + 1);
                }
            }
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut stack = Vec::new();

        for row in 0..rows {
            flood(&mut grid, row, 0, &mut stack);
            flood(&mut grid, row, cols - 1, &mut stack);
        }

        for col in 0..cols {
            flood(&mut grid, 0, col, &mut stack);
            flood(&mut grid, rows - 1, col, &mut stack);
        }

        let mut res = 0;

        for row in 1..rows.saturating_sub(1) {
            for col in 1..cols.saturating_sub(1) {
                if grid[row][col] == 0 {
                    res += 1;
                    flood(&mut grid, row, col, &mut stack);
                }
            }
        }

        res
    }
}
