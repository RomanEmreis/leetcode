/*
  3464. Maximize the Distance Between Points on a Square
  
  You are given an integer side, representing the edge length of a square with corners at (0, 0), (0, side), (side, 0), and (side, side) on a Cartesian plane.
  You are also given a positive integer k and a 2D integer array points, where points[i] = [xi, yi] represents the coordinate of a point lying on the boundary of the square.
  You need to select k elements among points such that the minimum Manhattan distance between any two points is maximized.
  
  Return the maximum possible minimum Manhattan distance between the selected k points.
  
  The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
  
  Example 1:
  Input: side = 2, points = [[0,2],[2,0],[2,2],[0,0]], k = 4
  Output: 2
  Explanation:
  Select all four points.
  
  Example 2:
  Input: side = 2, points = [[0,0],[1,2],[2,0],[2,2],[2,1]], k = 4
  Output: 1
  Explanation:
  Select the points (0, 0), (2, 0), (2, 2), and (2, 1).
  
  Example 3:
  Input: side = 2, points = [[0,0],[0,1],[0,2],[1,2],[2,0],[2,2],[2,1]], k = 5
  Output: 1
  Explanation:
  Select the points (0, 0), (0, 1), (0, 2), (1, 2), and (2, 2).
*/
impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let side = side as u32;
        let k = k as usize;
        let n = points.len();
        let perim = 4 * side as u64;

        #[inline(always)]
        fn to_pos(x: u32, y: u32, side: u32) -> u32 {
            if y == 0         { x }
            else if x == side { side + y }
            else if y == side { 3 * side - x }
            else              { 4 * side - y }
        }

        let mut pos: Vec<u32> = points
            .iter()
            .map(|p| to_pos(p[0] as u32, p[1] as u32, side))
            .collect();
        pos.sort_unstable();

        let mut prefix = vec![0u32; 2 * n + 1];
        for i in 1..n {
            prefix[i] = prefix[i - 1] + (pos[i] - pos[i - 1]);
        }
        let total = (prefix[n - 1] as u64
            + (perim - (pos[n - 1] - pos[0]) as u64)) as u32;
        for i in 0..n {
            prefix[n + i] = prefix[i] + total;
        }
        prefix[2 * n] = u32::MAX / 2;
        drop(pos);

        let len = 2 * n;
        let log = usize::BITS as usize - (k - 1).max(1).leading_zeros() as usize;
        let buf_len = len + 1;

        let mut a = vec![0u32; buf_len];
        let mut c = vec![0u32; buf_len];
        let mut accum = vec![0u32; n];

        let check = |d: u32,
                     a: &mut Vec<u32>,
                     c: &mut Vec<u32>,
                     accum: &mut Vec<u32>|
         -> bool {
            {
                let mut r = 0usize;
                for i in 0..len {
                    if r <= i { r = i + 1; }
                    while r < len && prefix[r] - prefix[i] < d { r += 1; }
                    unsafe { *a.get_unchecked_mut(i) = r as u32; }
                }
                a[len] = len as u32;
            }

            for i in 0..n {
                unsafe { *accum.get_unchecked_mut(i) = i as u32; }
            }

            let km1 = k - 1;
            for bit in 0..=log {
                if km1 & (1 << bit) != 0 {
                    for i in 0..n {
                        let mid = unsafe { *accum.get_unchecked(i) } as usize;
                        let val = unsafe { *a.get_unchecked(mid) };
                        unsafe { *accum.get_unchecked_mut(i) = val; }
                    }
                }
                if bit < log {
                    for i in 0..buf_len {
                        let mid = unsafe { *a.get_unchecked(i) } as usize;
                        let val = unsafe { *a.get_unchecked(mid) };
                        unsafe { *c.get_unchecked_mut(i) = val; }
                    }
                    std::mem::swap(a, c);
                }
            }

            for i in 0..n {
                let cur = unsafe { *accum.get_unchecked(i) } as usize;
                if cur < i + n {
                    let arc_back = total - (prefix[cur] - prefix[i]);
                    if arc_back >= d { return true; }
                }
            }
            false
        };

        let mut lo = 1u32;
        let mut hi = 2 * side;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if check(mid, &mut a, &mut c, &mut accum) { lo = mid; }
            else { hi = mid - 1; }
        }
        lo as i32
    }
}
