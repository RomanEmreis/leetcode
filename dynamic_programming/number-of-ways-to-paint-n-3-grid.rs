/*
  1411. Number of Ways to Paint N × 3 Grid
  
  You have a grid of size n x 3 and you want to paint each cell of the grid with exactly one of the three colors: Red, Yellow, 
  or Green while making sure that no two adjacent cells have the same color (i.e., no two cells that share vertical or horizontal sides have the same color).
  
  Given n the number of rows of the grid, return the number of ways you can paint this grid. As the answer may grow large, the answer must be computed modulo 109 + 7.
  
  Example 1:
  Input: n = 1
  Output: 12
  Explanation: There are 12 possible way to paint the grid as shown.
  
  Example 2:
  Input: n = 5000
  Output: 30228214
*/
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut red = 6;
        let mut blue = 6;
        for i in 1..n {
            let next_red = (2 * red + 2 * blue) % MOD;
            let next_blue = (2 * red + 3 * blue) % MOD;
            red = next_red;
            blue = next_blue;
        }
        ((red + blue) % MOD) as i32
    }
}
