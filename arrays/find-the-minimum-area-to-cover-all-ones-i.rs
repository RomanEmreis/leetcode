/*
  3195. Find the Minimum Area to Cover All Ones I
  
  You are given a 2D binary array grid. Find a rectangle with horizontal and vertical sides with the smallest area, such that all the 1's in grid lie inside this rectangle.
  
  Return the minimum possible area of the rectangle.
  
  Example 1:
  Input: grid = [[0,1,0],[1,0,1]]
  Output: 6
  Explanation:
  The smallest rectangle has a height of 2 and a width of 3, so it has an area of 2 * 3 = 6.
  
  Example 2:
  Input: grid = [[1,0],[0,0]]
  Output: 1
  Explanation:
  
  The smallest rectangle has both height and width 1, so its area is 1 * 1 = 1.
*/
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let (mut min_r, mut min_c) = (m as i32, n as i32);
        let (mut max_r, mut max_c) = (-1i32, -1i32);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    min_r = min_r.min(i as i32);
                    min_c = min_c.min(j as i32);
                    max_r = max_r.max(i as i32);
                    max_c = max_c.max(j as i32);
                }
            }
        }
        (1 + max_r - min_r) * (1 + max_c - min_c)
    }
}
