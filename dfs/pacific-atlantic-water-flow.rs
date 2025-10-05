/*
  417. Pacific Atlantic Water Flow
  
  There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. 
  The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
  
  The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
  The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height.
  Water can flow from any cell adjacent to an ocean into the ocean.
  
  Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
  
  Example 1:
  Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
  Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
  Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
  [0,4]: [0,4] -> Pacific Ocean 
         [0,4] -> Atlantic Ocean
  [1,3]: [1,3] -> [0,3] -> Pacific Ocean 
         [1,3] -> [1,4] -> Atlantic Ocean
  [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean 
         [1,4] -> Atlantic Ocean
  [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean 
         [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
  [3,0]: [3,0] -> Pacific Ocean 
         [3,0] -> [4,0] -> Atlantic Ocean
  [3,1]: [3,1] -> [3,0] -> Pacific Ocean 
         [3,1] -> [4,1] -> Atlantic Ocean
  [4,0]: [4,0] -> Pacific Ocean 
         [4,0] -> Atlantic Ocean
  Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
  
  Example 2:
  Input: heights = [[1]]
  Output: [[0,0]]
  Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
*/
use std::collections::VecDeque;

const DIRS: [(isize, isize); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0)
];

fn bfs(
    m: usize,
    n: usize,
    seen: &mut Vec<Vec<bool>>,
    heights: &Vec<Vec<i32>>,
    mut q: VecDeque<(usize, usize)>) { 
    while let Some(item) = q.pop_front() {
        let (i, j) = item;
        let h = heights[i][j];
        for (dx, dy) in DIRS.iter() {
            let x = i as isize + dx;
            let y = j as isize + dy;
            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;
            if x == m || y == n {
                continue;
            }
            if seen[x][y] || heights[x][y] < h {
                continue;
            }

            q.push_back((x, y));
            seen[x][y] = true;
        }
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut q_p = VecDeque::with_capacity(m + n);
        let mut q_a = VecDeque::with_capacity(m + n);
        let mut seen_p = vec![vec![false; n]; m];
        let mut seen_a = vec![vec![false; n]; m];
        for i in 0..m {
            q_p.push_back((i, 0));
            q_a.push_back((i, n - 1));
            seen_p[i][0] = true;
            seen_a[i][n - 1] = true;
        }
        for j in 0..n {
            q_p.push_back((0, j));
            q_a.push_back((m - 1, j));
            seen_p[0][j] = true;
            seen_a[m - 1][j] = true;
        }

        bfs(m, n, &mut seen_p, &heights, q_p);
        bfs(m, n, &mut seen_a, &heights, q_a);

        let mut res = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if seen_p[i][j] && seen_a[i][j] {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}
