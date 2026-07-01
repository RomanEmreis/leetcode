/*
  2812. Find the Safest Path in a Grid
  
  You are given a 0-indexed 2D matrix grid of size n x n, where (r, c) represents:
      A cell containing a thief if grid[r][c] = 1
      An empty cell if grid[r][c] = 0
  
  You are initially positioned at cell (0, 0). In one move, you can move to any adjacent cell in the grid, including cells containing thieves.
  The safeness factor of a path on the grid is defined as the minimum manhattan distance from any cell in the path to any thief in the grid.
  
  Return the maximum safeness factor of all paths leading to cell (n - 1, n - 1).
  
  An adjacent cell of cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) and (r - 1, c) if it exists.
  The Manhattan distance between two cells (a, b) and (x, y) is equal to |a - x| + |b - y|, where |val| denotes the absolute value of val.
  
  Example 1:
  Input: grid = [[1,0,0],[0,0,0],[0,0,1]]
  Output: 0
  Explanation: All paths from (0, 0) to (n - 1, n - 1) go through the thieves in cells (0, 0) and (n - 1, n - 1).
  
  Example 2:
  Input: grid = [[0,0,1],[0,0,0],[0,0,0]]
  Output: 2
  Explanation: The path depicted in the picture above has a safeness factor of 2 since:
  - The closest cell of the path to the thief at cell (0, 2) is cell (0, 0). The distance between them is | 0 - 0 | + | 0 - 2 | = 2.
  It can be shown that there are no other paths with a higher safeness factor.
  
  Example 3:
  Input: grid = [[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]]
  Output: 2
  Explanation: The path depicted in the picture above has a safeness factor of 2 since:
  - The closest cell of the path to the thief at cell (0, 3) is cell (1, 2). The distance between them is | 0 - 1 | + | 3 - 2 | = 2.
  - The closest cell of the path to the thief at cell (3, 0) is cell (3, 2). The distance between them is | 3 - 3 | + | 0 - 2 | = 2.
  It can be shown that there are no other paths with a higher safeness factor.
*/
impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let total = n * n;

        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return 0;
        }

        let mut dist = vec![-1i16; total];
        let mut queue = Vec::with_capacity(total);

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 1 {
                    let id = r * n + c;
                    dist[id] = 0;
                    queue.push(id);
                }
            }
        }

        if queue.is_empty() {
            return (2 * (n - 1)) as i32;
        }

        let mut head = 0;
        let mut max_dist = 0i16;

        while head < queue.len() {
            let id = queue[head];
            head += 1;

            let r = id / n;
            let c = id % n;
            let next = dist[id] + 1;

            if c + 1 < n {
                let nid = id + 1;
                if dist[nid] == -1 {
                    dist[nid] = next;
                    max_dist = max_dist.max(next);
                    queue.push(nid);
                }
            }

            if c > 0 {
                let nid = id - 1;
                if dist[nid] == -1 {
                    dist[nid] = next;
                    max_dist = max_dist.max(next);
                    queue.push(nid);
                }
            }

            if r + 1 < n {
                let nid = id + n;
                if dist[nid] == -1 {
                    dist[nid] = next;
                    max_dist = max_dist.max(next);
                    queue.push(nid);
                }
            }

            if r > 0 {
                let nid = id - n;
                if dist[nid] == -1 {
                    dist[nid] = next;
                    max_dist = max_dist.max(next);
                    queue.push(nid);
                }
            }
        }

        let mut count = vec![0usize; max_dist as usize + 1];

        for &d in &dist {
            count[d as usize] += 1;
        }

        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        let mut order = vec![0usize; total];

        for id in 0..total {
            let d = dist[id] as usize;
            count[d] -= 1;
            order[count[d]] = id;
        }

        let mut parent: Vec<usize> = (0..total).collect();
        let mut rank = vec![0u8; total];
        let mut active = vec![false; total];

        for &id in order.iter().rev() {
            active[id] = true;

            let r = id / n;
            let c = id % n;

            if c + 1 < n && active[id + 1] {
                Self::union(&mut parent, &mut rank, id, id + 1);
            }

            if c > 0 && active[id - 1] {
                Self::union(&mut parent, &mut rank, id, id - 1);
            }

            if r + 1 < n && active[id + n] {
                Self::union(&mut parent, &mut rank, id, id + n);
            }

            if r > 0 && active[id - n] {
                Self::union(&mut parent, &mut rank, id, id - n);
            }

            let start = Self::find(&mut parent, 0);
            let end = Self::find(&mut parent, total - 1);

            if start == end {
                return dist[id] as i32;
            }
        }

        0
    }

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }

        x
    }

    fn union(parent: &mut [usize], rank: &mut [u8], a: usize, b: usize) {
        let mut ra = Self::find(parent, a);
        let mut rb = Self::find(parent, b);

        if ra == rb {
            return;
        }

        if rank[ra] < rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        parent[rb] = ra;

        if rank[ra] == rank[rb] {
            rank[ra] += 1;
        }
    }
}
