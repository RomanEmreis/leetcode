/*
  1559. Detect Cycles in 2D Grid
  
  Given a 2D array of characters grid of size m x n, you need to find if there exists any cycle consisting of the same value in grid.
  
  A cycle is a path of length 4 or more in the grid that starts and ends at the same cell. 
  From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), 
  if it has the same value of the current cell.
  
  Also, you cannot move to the cell that you visited in your last move. For example, the cycle (1, 1) -> (1, 2) -> (1, 1) 
  is invalid because from (1, 2) we visited (1, 1) which was the last visited cell.
  
  Return true if any cycle of the same value exists in grid, otherwise, return false.
  
  Example 1:
  Input: grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
  Output: true
  Explanation: There are two valid cycles shown in different colors in the image below:
  
  Example 2:
  Input: grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
  Output: true
  Explanation: There is only one valid cycle highlighted in the image below:
  
  Example 3:
  Input: grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
  Output: false
*/
struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }
    
    #[inline]
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    
    #[inline]
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut dsu = DSU::new(m * n);
        
        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                let ch = grid[i][j];
                
                if i > 0 && grid[i - 1][j] == ch {
                    if !dsu.union(idx, (i - 1) * n + j) {
                        return true;
                    }
                }

                if j > 0 && grid[i][j - 1] == ch {
                    if !dsu.union(idx, i * n + j - 1) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
