/*
  2033. Minimum Operations to Make a Uni-Value Grid
  
  You are given a 2D integer grid of size m x n and an integer x. In one operation, 
  you can add x to or subtract x from any element in the grid.
  A uni-value grid is a grid where all the elements of it are equal.
  
  Return the minimum number of operations to make the grid uni-value. If it is not possible, return -1.
  
  Example 1:
  Input: grid = [[2,4],[6,8]], x = 2
  Output: 4
  Explanation: We can make every element equal to 4 by doing the following: 
  - Add x to 2 once.
  - Subtract x from 6 once.
  - Subtract x from 8 twice.
  A total of 4 operations were used.
  
  Example 2:
  Input: grid = [[1,5],[2,3]], x = 1
  Output: 5
  Explanation: We can make every element equal to 3.
  
  Example 3:
  Input: grid = [[1,2],[3,4]], x = 2
  Output: -1
  Explanation: It is impossible to make every element equal.
*/
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let n = grid.len() * grid[0].len();
        let first = grid[0][0];

        let mut arr = Vec::with_capacity(n);
        for a in grid.into_iter().flatten() {
            if (a - first) % x != 0 {
                return -1;
            }

            arr.push(a);
        }
        
        arr.sort_unstable();

        let median = arr[n / 2];
        arr.into_iter()
            .map(|e| (e - median).abs() / x)
            .sum()
    }
}
