/*
  3225. Maximum Score From Grid Operations
  
  You are given a 2D matrix grid of size n x n. Initially, all cells of the grid are colored white. In one operation, you can select any cell of indices (i, j),
  and color black all the cells of the jth column starting from the top row down to the ith row.
  The grid score is the sum of all grid[i][j] such that cell (i, j) is white and it has a horizontally adjacent black cell.
  
  Return the maximum score that can be achieved after some number of operations.
  
  Example 1:
  Input: grid = [[0,0,0,0,0],[0,0,3,0,0],[0,1,0,0,0],[5,0,0,3,0],[0,0,0,0,2]]
  Output: 11
  Explanation:
  In the first operation, we color all cells in column 1 down to row 3, and in the second operation, we color all cells in column 4 down to the last row.
  The score of the resulting grid is grid[3][0] + grid[1][2] + grid[3][3] which is equal to 11.
  
  Example 2:
  Input: grid = [[10,9,0,0,15],[7,1,0,8,0],[5,20,0,11,0],[0,0,0,1,2],[8,12,1,10,3]]
  Output: 94
  Explanation:
  We perform operations on 1, 2, and 3 down to rows 1, 4, and 0, respectively. 
  The score of the resulting grid is grid[0][0] + grid[1][0] + grid[2][1] + grid[4][1] + grid[1][3] + grid[2][3] + grid[3][3] + grid[4][3] + grid[0][4] which is equal to 94.
*/
const NEG: i64 = i64::MIN / 4;

impl Solution {

    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = n + 1;
 
        let mut col_sum = vec![0i64; n * m];
        for c in 0..n {
            let mut s = 0i64;
            for r in 0..n {
                s += i64::from(grid[r][c]);
                col_sum[c * m + r + 1] = s;
            }
        }

        let mut dp = vec![NEG; m * m];
        for h0 in 0..m {
            dp[h0] = 0;
        }
 
        let mut ndp = vec![NEG; m * m];
        let mut prefix_max = vec![NEG; m + 1];
        let mut suffix_max = vec![NEG; m + 1];
 
        for c in 0..n.saturating_sub(1) {
            let cs = &col_sum[c * m..(c + 1) * m];
 
            ndp.fill(NEG);
 
            for h_cur in 0..m {
                prefix_max[0] = NEG;
                for k in 0..m {
                    let v = dp[k * m + h_cur];
                    prefix_max[k + 1] = prefix_max[k].max(v);
                }
 
                suffix_max[m] = NEG;
                for k in (0..m).rev() {
                    let v = dp[k * m + h_cur];
                    let term = if v == NEG {
                        NEG
                    } else {
                        v + cs[k.max(h_cur)]
                    };
                    suffix_max[k] = suffix_max[k + 1].max(term);
                }
 
                let cs_h_cur = cs[h_cur];
                for h_next in 0..m {
                    let h_max_a = h_next.max(h_cur);
                    let add_a = cs[h_max_a] - cs_h_cur;
                    let pa = prefix_max[h_next + 1];
                    let cand_a = if pa == NEG { NEG } else { pa + add_a };
 
                    let qb = if h_next + 1 <= m {
                        suffix_max[h_next + 1]
                    } else {
                        NEG
                    };
                    let cand_b = if qb == NEG { NEG } else { qb - cs_h_cur };
 
                    ndp[h_cur * m + h_next] = cand_a.max(cand_b);
                }
            }
 
            std::mem::swap(&mut dp, &mut ndp);
        }
 
        if n == 1 {
            return *dp[..m].into_iter().max().unwrap();
        }
 
        let cs_last = &col_sum[(n - 1) * m..n * m];
        let mut best = 0i64;
        for h_left in 0..m {
            for h_cur in 0..m {
                let v = dp[h_left * m + h_cur];
                if v == NEG {
                    continue;
                }
                let h_max = h_left.max(h_cur);
                let add = cs_last[h_max] - cs_last[h_cur];
                if v + add > best {
                    best = v + add;
                }
            }
        }
        best
    }
}
