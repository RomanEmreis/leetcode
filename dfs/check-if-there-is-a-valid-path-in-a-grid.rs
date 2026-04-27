/*
  1391. Check if There is a Valid Path in a Grid
  
  You are given an m x n grid. Each cell of grid represents a street. The street of grid[i][j] can be:
      1 which means a street connecting the left cell and the right cell.
      2 which means a street connecting the upper cell and the lower cell.
      3 which means a street connecting the left cell and the lower cell.
      4 which means a street connecting the right cell and the lower cell.
      5 which means a street connecting the left cell and the upper cell.
      6 which means a street connecting the right cell and the upper cell.
  
  You will initially start at the street of the upper-left cell (0, 0). 
  A valid path in the grid is a path that starts from the upper left cell (0, 0) 
  and ends at the bottom-right cell (m - 1, n - 1). The path should only follow the streets.
  
  Notice that you are not allowed to change any street.
  
  Return true if there is a valid path in the grid or false otherwise.
  
  Example 1:
  Input: grid = [[2,4,3],[6,5,2]]
  Output: true
  Explanation: As shown you can start at cell (0, 0) and visit all the cells of the grid to reach (m - 1, n - 1).
  
  Example 2:
  Input: grid = [[1,2,1],[1,2,1]]
  Output: false
  Explanation: As shown you the street at cell (0, 0) is not connected with any street of any other cell
  and you will get stuck at cell (0, 0)
  
  Example 3:
  Input: grid = [[1,1,2]]
  Output: false
  Explanation: You will get stuck at cell (0, 1) and you cannot reach cell (0, 2).
*/
use std::collections::VecDeque;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;

        if total == 1 {
            return true;
        }

        let mut visited = vec![0u64; (total + 63) / 64];
        let mark = |v: &mut [u64], idx: usize| v[idx >> 6] |= 1u64 << (idx & 63);
        let is_set = |v: &[u64], idx: usize| (v[idx >> 6] >> (idx & 63)) & 1 == 1;

        let mut queue: VecDeque<u32> = VecDeque::new();

        mark(&mut visited, 0);
        queue.push_back(0);

        let neighbors: [(i32, u8, u8); 4] = [
            (-1, L, R),       // left:  dc = -1
            (1, R, L),        // right: dc = +1
            (-(n as i32), U, D), // up:    dr = -1, flat delta = -n
            (n as i32, D, U), // down:  dr = +1, flat delta = +n
        ];

        let target = (total - 1) as u32;

        while let Some(idx) = queue.pop_front() {
            let r = (idx as usize) / n;
            let c = (idx as usize) % n;
            let cur = DIRS[grid[r][c] as usize];

            for &(delta, self_dir, neigh_dir) in &neighbors {
                if cur & self_dir == 0 {
                    continue;
                }

                let nidx = idx as i32 + delta;
                if nidx < 0 || nidx >= total as i32 {
                    continue;
                }

                let nidx = nidx as usize;
                if (self_dir == L || self_dir == R) && nidx / n != r {
                    continue;
                }

                if is_set(&visited, nidx) {
                    continue;
                }

                if DIRS[grid[nidx / n][nidx % n] as usize] & neigh_dir == 0 {
                    continue;
                }

                if nidx as u32 == target {
                    return true;
                }

                mark(&mut visited, nidx);
                queue.push_back(nidx as u32);
            }
        }

        false
    }
}

const L: u8 = 1;
const R: u8 = 2;
const U: u8 = 4;
const D: u8 = 8;

const DIRS: [u8; 7] = [0, L | R, U | D, L | D, R | D, L | U, R  | U];
