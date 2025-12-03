/*
  3625. Count Number of Trapezoids II
  
  You are given a 2D integer array points where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
  
  Return the number of unique trapezoids that can be formed by choosing any four distinct points from points.
  
  A trapezoid is a convex quadrilateral with at least one pair of parallel sides. Two lines are parallel if and only if they have the same slope.
  
  Example 1:
  Input: points = [[-3,2],[3,0],[2,3],[3,2],[2,-3]]
  Output: 2
  Explanation:
  There are two distinct ways to pick four points that form a trapezoid:
      The points [-3,2], [2,3], [3,2], [2,-3] form one trapezoid.
      The points [2,3], [3,2], [3,0], [2,-3] form another trapezoid.
  
  Example 2:
  Input: points = [[0,0],[1,0],[0,1],[2,1]]
  Output: 1
  Explanation:
  There is only one trapezoid which can be formed.
*/
use std::collections::HashMap;

const MUL: i64 = 1000000;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut slope_to_intercept: HashMap<(i64, i64), Vec<i64>> = HashMap::new();
        let mut mid_to_slope: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();

        for i in 0..n {
            let x1 = points[i][0] as i64;
            let y1 = points[i][1] as i64;
            for j in i + 1..n {
                let x2 = points[j][0] as i64;
                let y2 = points[j][1] as i64;
                
                let dx = x2 - x1;
                let dy = y2 - y1;

                let (slope_num, slope_den) = if dx == 0 {
                    (1, 0)
                } else {
                    let g = gcd(dy.abs(), dx.abs());
                    let mut num = dy / g;
                    let mut den = dx / g;
                    
                    if den < 0 {
                        num = -num;
                        den = -den;
                    }
                    (num, den)
                };

                let intercept = if dx == 0 {
                    x1 * MUL
                } else {
                    ((y1 * dx - dy * x1) * MUL) / dx
                };

                let mid = ((x1 + x2) * MUL, (y1 + y2) * MUL);

                slope_to_intercept
                    .entry((slope_num, slope_den))
                    .or_insert_with(Vec::new)
                    .push(intercept);

                mid_to_slope
                    .entry(mid)
                    .or_insert_with(Vec::new)
                    .push((slope_num, slope_den));
            }
        }

        let mut res = 0;

        for (_, sti) in slope_to_intercept.into_iter() {
            if sti.len() == 1 {
                continue;
            }

            let mut cnt: HashMap<i64, i32> = HashMap::new();
            for val in sti.into_iter() {
                *cnt.entry(val).or_insert(0) += 1;
            }
            
            let mut total = 0;
            for (_, c) in cnt.into_iter() {
                res += total * c;
                total += c;
            }
        }

        for (_, mts) in mid_to_slope.into_iter() {
            if mts.len() == 1 {
                continue;
            }

            let mut cnt: HashMap<(i64, i64), i32> = HashMap::new();
            for val in mts.into_iter() {
                *cnt.entry(val).or_insert(0) += 1;
            }
            
            let mut total = 0;
            for (_, c) in cnt.into_iter() {
                res -= total * c;
                total += c;
            }
        }

        res
    }
}

#[inline(always)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
