/*
  3454. Separate Squares II
  
  You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
  Find the minimum y-coordinate value of a horizontal line such that the total area covered by squares above the line equals the total area covered by squares below the line.
  
  Answers within 10-5 of the actual answer will be accepted.
  Note: Squares may overlap. Overlapping areas should be counted only once in this version.
  
  Example 1:
  Input: squares = [[0,0,1],[2,2,1]]
  Output: 1.00000
  Explanation:
  Any horizontal line between y = 1 and y = 2 results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
  
  Example 2:
  Input: squares = [[0,0,2],[1,1,1]]
  Output: 1.00000
  Explanation:
  Since the blue square overlaps with the red square, it will not be counted again. Thus, the line y = 1 splits the squares into two equal parts.
*/
use std::cmp::Ordering;
use std::collections::BTreeSet;

struct SegmentTree {
    xs: Vec<i32>,
    n: usize,
    covered_count: Vec<i32>,
    covered_width: Vec<i32>,
}

impl SegmentTree {
    fn new(xs: Vec<i32>) -> Self {
        let n = xs.len() - 1;
        Self {
            xs,
            n,
            covered_count: vec![0; 4 * n],
            covered_width: vec![0; 4 * n],
        }
    }

    fn add(&mut self, l: i32, r: i32, val: i32) {
        self.add_impl(0, 0, self.n - 1, l, r, val);
    }

    fn covered_width(&self) -> i32 {
        self.covered_width[0]
    }

    fn add_impl(
        &mut self,
        idx: usize,
        lo: usize,
        hi: usize,
        l: i32,
        r: i32,
        val: i32,
    ) {
        if r <= self.xs[lo] || self.xs[hi + 1] <= l {
            return;
        }

        if l <= self.xs[lo] && self.xs[hi + 1] <= r {
            self.covered_count[idx] += val;
        } else {
            let mid = (lo + hi) / 2;
            self.add_impl(idx * 2 + 1, lo, mid, l, r, val);
            self.add_impl(idx * 2 + 2, mid + 1, hi, l, r, val);
        }

        if self.covered_count[idx] > 0 {
            self.covered_width[idx] = self.xs[hi + 1] - self.xs[lo];
        } else if lo == hi {
            self.covered_width[idx] = 0;
        } else {
            self.covered_width[idx] =
                self.covered_width[idx * 2 + 1]
                + self.covered_width[idx * 2 + 2];
        }
    }
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut events: Vec<(i32, i32, i32, i32)> = Vec::new();
        let mut xs: BTreeSet<i32> = BTreeSet::new();

        for sq in squares {
            let x = sq[0];
            let y = sq[1];
            let l = sq[2];

            events.push((y, 1, x, x + l));
            events.push((y + l, -1, x, x + l));

            xs.insert(x);
            xs.insert(x + l);
        }

        events.sort_by(|a, b| a.0.cmp(&b.0));

        let xs: Vec<i32> = xs.into_iter().collect();
        let half_area = Self::total_area(&events, &xs) / 2.0;

        let mut tree = SegmentTree::new(xs);
        let mut area: f64 = 0.0;
        let mut prev_y = events[0].0;

        for (y, delta, xl, xr) in events {
            let width = tree.covered_width() as f64;
            let dy = (y - prev_y) as f64;
            let area_gain = width * dy;

            if area + area_gain >= half_area {
                return prev_y as f64 + (half_area - area) / width;
            }

            area += area_gain;
            tree.add(xl, xr, delta);
            prev_y = y;
        }

        unreachable!()
    }

    fn total_area(events: &[(i32, i32, i32, i32)], xs: &[i32]) -> f64 {
        let mut tree = SegmentTree::new(xs.to_vec());
        let mut area: f64 = 0.0;
        let mut prev_y = events[0].0;

        for &(y, delta, xl, xr) in events {
            let width = tree.covered_width() as f64;
            area += width * (y - prev_y) as f64;
            tree.add(xl, xr, delta);
            prev_y = y;
        }

        area as f64
    }
}
