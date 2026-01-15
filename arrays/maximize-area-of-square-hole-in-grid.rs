/*
  2943. Maximize Area of Square Hole in Grid
  
  You are given the two integers, n and m and two integer arrays, hBars and vBars. The grid has n + 2 horizontal and m + 2 
  vertical bars, creating 1 x 1 unit cells. The bars are indexed starting from 1.
  You can remove some of the bars in hBars from horizontal bars and some of the bars in vBars from vertical bars. 

  Note that other bars are fixed and cannot be removed.
  
  Return an integer denoting the maximum area of a square-shaped hole in the grid, after removing some bars (possibly none).
  
  Example 1:
  Input: n = 2, m = 1, hBars = [2,3], vBars = [2]
  Output: 4
  Explanation:
  The left image shows the initial grid formed by the bars. The horizontal bars are [1,2,3,4], and the vertical bars are [1,2,3].
  One way to get the maximum square-shaped hole is by removing horizontal bar 2 and vertical bar 2.
  
  Example 2:
  Input: n = 1, m = 1, hBars = [2], vBars = [2]
  Output: 4
  Explanation:
  To get the maximum square-shaped hole, we remove horizontal bar 2 and vertical bar 2.
  
  Example 3:
  Input: n = 2, m = 3, hBars = [2,3], vBars = [2,4]
  Output: 4
  Explanation:
  One way to get the maximum square-shaped hole is by removing horizontal bar 3, and vertical bar 4.
*/
impl Solution {
    pub fn maximize_square_hole_area(_n: i32, _m: i32, mut h_bars: Vec<i32>, mut v_bars: Vec<i32>) -> i32 {
        let gap = get_gap(&mut h_bars).min(get_gap(&mut v_bars));
        gap * gap
    }
}

#[inline]
fn get_gap(bars: &mut [i32]) -> i32 {
    bars.sort();

    let mut res = 2;
    let mut seq = 2;

    for i in 1..bars.len() {
        seq = if bars[i] == bars[i - 1] + 1 {
            seq + 1
        } else {
            2
        };

        res = res.max(seq);
    }

    res
}
