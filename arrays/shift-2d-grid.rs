/*
  1260. Shift 2D Grid
  
  Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
  In one shift operation:
      Element at grid[i][j] moves to grid[i][j + 1].
      Element at grid[i][n - 1] moves to grid[i + 1][0].
      Element at grid[m - 1][n - 1] moves to grid[0][0].
  
  Return the 2D grid after applying shift operation k times.
  
  Example 1:
  Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
  Output: [[9,1,2],[3,4,5],[6,7,8]]
  
  Example 2:
  Input: grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
  Output: [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
  
  Example 3:
  Input: grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
  Output: [[1,2,3],[4,5,6],[7,8,9]]                                            
*/
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let total = rows * cols;
        let shift = k as usize % total;

        if shift == 0 {
            return grid;
        }

        let start = total - shift;
        let mut source_row = start / cols;
        let mut source_col = start % cols;

        let mut result = vec![vec![0; cols]; rows];

        for destination_row in &mut result {
            let mut destination_col = 0;

            while destination_col < cols {
                let count =
                    (cols - destination_col).min(cols - source_col);

                destination_row[destination_col..destination_col + count]
                    .copy_from_slice(
                        &grid[source_row][source_col..source_col + count],
                    );

                destination_col += count;
                source_col += count;

                if source_col == cols {
                    source_col = 0;
                    source_row += 1;

                    if source_row == rows {
                        source_row = 0;
                    }
                }
            }
        }

        result
    }
}
