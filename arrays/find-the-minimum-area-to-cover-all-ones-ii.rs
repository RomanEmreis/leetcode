/*
  3197. Find the Minimum Area to Cover All Ones II
  
  You are given a 2D binary array grid. You need to find 3 non-overlapping rectangles having non-zero areas with horizontal and vertical sides such that all the 1's in grid lie inside these rectangles.
  
  Return the minimum possible sum of the area of these rectangles.
  
  Note that the rectangles are allowed to touch.
  
  Example 1:
  Input: grid = [[1,0,1],[1,1,1]]
  Output: 5
  Explanation:
      The 1's at (0, 0) and (1, 0) are covered by a rectangle of area 2.
      The 1's at (0, 2) and (1, 2) are covered by a rectangle of area 2.
      The 1 at (1, 1) is covered by a rectangle of area 1.
  
  Example 2:
  Input: grid = [[1,0,1,0],[0,1,0,1]]
  Output: 5
  Explanation:
      The 1's at (0, 0) and (0, 2) are covered by a rectangle of area 3.
      The 1 at (1, 1) is covered by a rectangle of area 1.
      The 1 at (1, 3) is covered by a rectangle of area 1.
*/
impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = (m * n) as i32;
        for i in 0..m {
            let top = Self::min_area(&grid, 0, i, 0, n - 1);
            for j in 0..n {
                result = result.min(
                    top
                        + Self::min_area(&grid, i + 1, m.saturating_sub(1), 0, j)
                        + Self::min_area(&grid, i + 1, m.saturating_sub(1), j + 1, n - 1),
                );
            }
        }
        for i in 0..m {
            let bottom = Self::min_area(&grid, i, m - 1, 0, n - 1);
            for j in 0..n {
                result = result.min(
                    bottom
                        + if i > 0 { Self::min_area(&grid, 0, i - 1, 0, j) } else { 0 }
                        + if i > 0 {
                            Self::min_area(&grid, 0, i - 1, j + 1, n - 1)
                        } else {
                            0
                        },
                );
            }
        }
        for j in 0..n {
            let left = Self::min_area(&grid, 0, m - 1, 0, j);
            for i in 0..m {
                result = result.min(
                    left
                        + Self::min_area(&grid, 0, i, j + 1, n - 1)
                        + Self::min_area(&grid, i + 1, m - 1, j + 1, n - 1),
                );
            }
        }
        for j in 0..n {
            let right = Self::min_area(&grid, 0, m - 1, j, n - 1);
            for i in 0..m {
                result = result.min(
                    right
                        + if j > 0 { Self::min_area(&grid, 0, i, 0, j - 1) } else { 0 }
                        + if j > 0 {
                            Self::min_area(&grid, i + 1, m - 1, 0, j - 1)
                        } else {
                            0
                        },
                );
            }
        }
        for i1 in 0..m {
            for i2 in i1 + 1..m {
                result = result.min(
                    Self::min_area(&grid, 0, i1, 0, n - 1)
                        + Self::min_area(&grid, i1 + 1, i2, 0, n - 1)
                        + Self::min_area(&grid, i2 + 1, m - 1, 0, n - 1),
                );
            }
        }
        for j1 in 0..n {
            for j2 in j1 + 1..n {
                result = result.min(
                    Self::min_area(&grid, 0, m - 1, 0, j1)
                        + Self::min_area(&grid, 0, m - 1, j1 + 1, j2)
                        + Self::min_area(&grid, 0, m - 1, j2 + 1, n - 1),
                );
            }
        }
        result   
    }

    fn min_area(
        grid: &Vec<Vec<i32>>, 
        si: usize, 
        ei: usize, 
        sj: usize, 
        ej: usize
    ) -> i32 {
        if si > ei || sj > ej {
            return 0;
        }

        let mut min_r = i32::MAX;
        let mut min_c = i32::MAX;
        let mut max_r = 0;
        let mut max_c = 0;

        for i in si..=ei {
            for j in sj..=ej {
                if grid[i][j] == 1 {
                    min_r = min_r.min(i as i32);
                    min_c = min_c.min(j as i32);
                    max_r = max_r.max(i as i32);
                    max_c = max_c.max(j as i32);
                }
            }
        }

        if min_r == i32::MAX {
            0
        } else {
            (1 + max_r - min_r) * (1 + max_c - min_c)
        }
    }
}
