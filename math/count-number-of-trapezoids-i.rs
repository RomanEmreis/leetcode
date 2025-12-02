/*
  3623. Count Number of Trapezoids I
  
  You are given a 2D integer array points, where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
  A horizontal trapezoid is a convex quadrilateral with at least one pair of horizontal sides (i.e. parallel to the x-axis). 
  Two lines are parallel if and only if they have the same slope.
  
  Return the number of unique horizontal trapezoids that can be formed by choosing any four distinct points from points.
  
  Since the answer may be very large, return it modulo 10^9 + 7.
  
  Example 1:
  Input: points = [[1,0],[2,0],[3,0],[2,2],[3,2]]
  Output: 3
  Explanation:
  There are three distinct ways to pick four points that form a horizontal trapezoid:
      Using points [1,0], [2,0], [3,2], and [2,2].
      Using points [2,0], [3,0], [3,2], and [2,2].
      Using points [1,0], [3,0], [3,2], and [2,2].
  
  Example 2:
  Input: points = [[0,0],[1,0],[0,1],[2,1]]
  Output: 1
  Explanation:
  There is only one horizontal trapezoid that can be formed.
*/
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut freq = HashMap::with_capacity(points.len());
        for p in points.into_iter() {
            *freq.entry(p[1] as i64).or_insert(0) += 1;
        }

        let mut res: i64 = 0;
        let mut sum: i64 = 0;

        for f in freq.values() {
            let e = f * (f - 1) / 2;
            res = (res + e * sum) % MOD;
            sum = (sum + e) % MOD;
        }

        res as i32
    }
}
