/*
  3286. Find a Safe Walk Through a Grid
  
  You are given an m x n binary matrix grid and an integer health.
  You start on the upper-left corner (0, 0) and would like to get to the lower-right corner (m - 1, n - 1).
  You can move up, down, left, or right from one cell to another adjacent cell as long as your health remains positive.
  
  Cells (i, j) with grid[i][j] = 1 are considered unsafe and reduce your health by 1.
  
  Return true if you can reach the final cell with a health value of 1 or more, and false otherwise.
  
  Example 1:
  Input: grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]], health = 1
  Output: true
  Explanation:
  The final cell can be reached safely by walking along the gray cells below.
  
  Example 2:
  Input: grid = [[0,1,1,0,0,0],[1,0,1,0,0,0],[0,1,1,1,0,1],[0,0,1,0,1,0]], health = 3
  Output: false
  Explanation:
  A minimum of 4 health points is needed to reach the final cell safely.
  
  Example 3:
  Input: grid = [[1,1,1],[1,0,1],[1,1,1]], health = 5
  Output: true
  Explanation:
  The final cell can be reached safely by walking along the gray cells below.
  Any path that does not go through the cell (1, 1) is unsafe since your health will drop to 0 when reaching the final cell.
*/
use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let target = m * n - 1;

        let mut dist = vec![i32::MAX; m * n];
        let mut deque = VecDeque::with_capacity(m * n);

        dist[0] = grid[0][0];
        deque.push_back(0usize);

        while let Some(idx) = deque.pop_front() {
            let d = dist[idx]; 
            if idx == target {
                return health > d;
            }
            let (r, c) = (idx / n, idx % n);

            let mut relax = |nr: usize, nc: usize| {
                let nidx = nr * n + nc;
                let w = grid[nr][nc];
                if d + w < dist[nidx] {
                    dist[nidx] = d + w;
                    if w == 0 {
                        deque.push_front(nidx);
                    } else {
                        deque.push_back(nidx);
                    }
                }
            };

            if r > 0 { 
                relax(r - 1, c);
            }
            if r + 1 < m { 
                relax(r + 1, c);
            }
            if c > 0 { 
                relax(r, c - 1);
            }
            if c + 1 < n { 
                relax(r, c + 1);
            }
        }

        health > dist[target]
    }
}
