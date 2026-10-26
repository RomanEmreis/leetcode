/*
  3619. Count Islands With Total Value Divisible by K
  
  You are given an m x n matrix grid and a positive integer k. 
  An island is a group of positive integers (representing land) that are 4-directionally connected (horizontally or vertically).
  
  The total value of an island is the sum of the values of all cells in the island.
  
  Return the number of islands with a total value divisible by k.
  
  Example 1:
  Input: grid = [[0,2,1,0,0],[0,5,0,0,5],[0,0,1,0,0],[0,1,4,7,0],[0,2,0,0,8]], k = 5
  Output: 2
  Explanation:
  The grid contains four islands. The islands highlighted in blue have a total value that is divisible by 5, while the islands highlighted in red do not.
  
  Example 2:
  Input: grid = [[3,0,3,0], [0,3,0,3], [3,0,3,0]], k = 3
  Output: 6
  Explanation:
  The grid contains six islands, each with a total value that is divisible by 3.
*/
impl Solution {
    pub fn count_islands(
        mut grid: Vec<Vec<i32>>,
        k: i32,
    ) -> i32 {
        fn flood(
            grid: &mut [Vec<i32>],
            start: usize,
            k: i32,
            stack: &mut Vec<u32>,
        ) -> i32 {
            let rows = grid.len();
            let cols = grid[0].len();

            let start_row = start / cols;
            let start_col = start % cols;

            let mut remainder = grid[start_row][start_col] % k;

            grid[start_row][start_col] = 0;
            stack.push(start as u32);

            while let Some(position) = stack.pop() {
                let position = position as usize;
                let row = position / cols;
                let col = position % cols;

                if row > 0 && grid[row - 1][col] > 0 {
                    remainder = (remainder + grid[row - 1][col]) % k;

                    grid[row - 1][col] = 0;
                    stack.push(((row - 1) * cols + col) as u32);
                }

                if row + 1 < rows && grid[row + 1][col] > 0 {
                    remainder = (remainder + grid[row + 1][col]) % k;

                    grid[row + 1][col] = 0;
                    stack.push(((row + 1) * cols + col) as u32);
                }

                if col > 0 && grid[row][col - 1] > 0 {
                    remainder = (remainder + grid[row][col - 1]) % k;

                    grid[row][col - 1] = 0;
                    stack.push((row * cols + col - 1) as u32);
                }

                if col + 1 < cols && grid[row][col + 1] > 0 {
                    remainder = (remainder + grid[row][col + 1]) % k;

                    grid[row][col + 1] = 0;
                    stack.push((row * cols + col + 1) as u32);
                }
            }

            remainder
        }

        let rows = grid.len();
        let cols = grid[0].len();

        let mut stack = Vec::<u32>::with_capacity(rows * cols);
        let mut res = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] > 0
                    && flood(
                        &mut grid,
                        row * cols + col,
                        k,
                        &mut stack,
                    ) == 0
                {
                    res += 1;
                }
            }
        }

        res
    }
}
