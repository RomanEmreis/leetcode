/*
  407. Trapping Rain Water II
  
  Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
  
  Example 1:
  Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
  Output: 4
  Explanation: After the rain, water is trapped between the blocks.
  We have two small ponds 1 and 3 units trapped.
  The total volume of water trapped is 4.
  
  Example 2:
  Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0)
];

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        let mut min_heap = BinaryHeap::new();
        let mut seen = vec![vec![false; n]; m];
        let mut res = 0;

        for i in 0..m {
            min_heap.push((Reverse(height_map[i][0]), i, 0));
            min_heap.push((Reverse(height_map[i][n - 1]), i, n - 1));
            seen[i][0] = true;
            seen[i][n - 1] = true;
        }
        for j in 1..n - 1 {
            min_heap.push((Reverse(height_map[0][j]), 0, j));
            min_heap.push((Reverse(height_map[m - 1][j]), m - 1, j));
            seen[0][j] = true;
            seen[m - 1][j] = true;
        }
        while let Some(item) = min_heap.pop() {
            let (h, i, j) = item;
            let h = h.0;
            for (dx, dy) in DIRS {
                let x = i as i32 + dx;
                let y = j as i32 + dy;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if  x == m || y == n {
                    continue;
                }

                if seen[x][y] {
                    continue;
                }
                
                let height = height_map[x][y];

                if height < h {
                    res += h - height;
                    min_heap.push((Reverse(h), x, y));
                } else {
                    min_heap.push((Reverse(height), x, y));
                }

                seen[x][y] = true;
            }
        }

        res
    }
}
