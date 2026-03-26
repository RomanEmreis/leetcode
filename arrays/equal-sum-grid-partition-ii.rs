/*
  3548. Equal Sum Grid Partition II
  
  You are given an m x n matrix grid of positive integers. Your task is to determine if it is possible 
  to make either one horizontal or one vertical cut on the grid such that:
      Each of the two resulting sections formed by the cut is non-empty.
      The sum of elements in both sections is equal, or can be made equal by discounting at most one single cell in total (from either section).
      If a cell is discounted, the rest of the section must remain connected.
  
  Return true if such a partition exists; otherwise, return false.
  
  Note: A section is connected if every cell in it can be reached from any other cell by moving up, down, left, or right through other cells in the section.
  
  Example 1:
  Input: grid = [[1,4],[2,3]]
  Output: true
  Explanation:
      A horizontal cut after the first row gives sums 1 + 4 = 5 and 2 + 3 = 5, which are equal. Thus, the answer is true.
  
  Example 2:
  Input: grid = [[1,2],[3,4]]
  Output: true
  Explanation:
      A vertical cut after the first column gives sums 1 + 3 = 4 and 2 + 4 = 6.
      By discounting 2 from the right section (6 - 2 = 4), both sections have equal sums and remain connected. Thus, the answer is true.
  
  Example 3:
  Input: grid = [[1,2,4],[2,3,5]]
  Output: false
  Explanation:
      A horizontal cut after the first row gives 1 + 2 + 4 = 7 and 2 + 3 + 5 = 10.
      By discounting 3 from the bottom section (10 - 3 = 7), both sections have equal sums, but they do not remain connected
      as it splits the bottom section into two parts ([2] and [5]). Thus, the answer is false.
  
  Example 4:
  Input: grid = [[4,1,8],[3,2,6]]
  Output: false
  Explanation:
  No valid cut exists, so the answer is false.
*/
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        Self::check(&grid) || Self::check(&Self::transpose(&grid))
    }

    fn check(g: &[Vec<i32>]) -> bool {
        let m = g.len();
        let n = g[0].len();
        const MAX_VAL: usize = 100_001;
        
        let mut cnt2 = vec![0_i32; MAX_VAL];
        let mut cnt1 = vec![0_i32; MAX_VAL];

        let mut total = 0_i64;
        for row in g.iter() {
            for &x in row.iter() {
                cnt2[x as usize] += 1;
                total += x as i64;
            }
        }

        let mut s1 = 0_i64;
        let mut s2 = total;

        for i in 0..m - 1 {
            for &x in g[i].iter() {
                let v = x as usize;
                s1 += x as i64;
                s2 -= x as i64;
                cnt1[v] += 1;
                cnt2[v] -= 1;
            }

            if s1 == s2 {
                return true;
            }

            let (diff_i64, in_bottom) = if s1 < s2 {
                (s2 - s1, true)
            } else {
                (s1 - s2, false)
            };

            if diff_i64 <= 0 || diff_i64 >= MAX_VAL as i64 {
                continue;
            }
            let diff = diff_i64 as usize;

            let cnt = if in_bottom { &cnt2 } else { &cnt1 };
            if cnt[diff] == 0 {
                continue;
            }

            let rows_in_section = if in_bottom { m - i - 1 } else { i + 1 };

            let valid = if n == 1 {
                if in_bottom {
                    g[i + 1][0] as usize == diff || g[m - 1][0] as usize == diff
                } else {
                    g[0][0] as usize == diff || g[i][0] as usize == diff
                }
            } else if rows_in_section == 1 {
                let row = if in_bottom { i + 1 } else { i };
                g[row][0] as usize == diff || g[row][n - 1] as usize == diff
            } else {
                true
            };

            if valid {
                return true;
            }
        }

        false
    }

    fn transpose(g: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let m = g.len();
        let n = g[0].len();
        (0..n)
            .map(|j| (0..m).map(|i| g[i][j]).collect())
            .collect()
    }
}
