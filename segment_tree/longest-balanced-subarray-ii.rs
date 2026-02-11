/*
  3721. Longest Balanced Subarray II
  
  You are given an integer array nums.
  A is called balanced if the number of distinct even numbers in the subarray is equal to the number of distinct odd numbers.
  
  Return the length of the longest balanced subarray.
  
  Example 1:
  Input: nums = [2,5,4,3]
  Output: 4
  Explanation:
      The longest balanced subarray is [2, 5, 4, 3].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [5, 3]. Thus, the answer is 4.
  
  Example 2:
  Input: nums = [3,2,2,5,4]
  Output: 5
  Explanation:
      The longest balanced subarray is [3, 2, 2, 5, 4].
      It has 2 distinct even numbers [2, 4] and 2 distinct odd numbers [3, 5]. Thus, the answer is 5.
  
  Example 3:
  Input: nums = [1,2,3,2]
  Output: 3
  Explanation:
      The longest balanced subarray is [2, 3, 2].
      It has 1 distinct even number [2] and 1 distinct odd number [3]. Thus, the answer is 3.
*/
use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut st = SegmentTree::new(n);

        let mut now = 0;
        let mut res = 0;
        let mut last: HashMap<i32, usize> = HashMap::new();

        for i in 1..=n {
            let x = unsafe { *nums.get_unchecked(i - 1) };
            let dt = if (x & 1) > 0 { 1 } else { -1 };

            if let Some(&l) = last.get(&x) {
                st.modify(1, l, n, -dt);
                now -= dt;
            }

            last.insert(x, i);
            st.modify(1, i, n, dt);
            now += dt;

            let j = st.query(1, now);
            res = res.max(i - j);
        }

        res as i32
    }
}

struct SegmentTree {
    l: Vec<usize>,
    r: Vec<usize>,
    min: Vec<i32>,
    max: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    #[inline]
    fn new(n: usize) -> Self {
        let size = n << 2;
        let mut st = SegmentTree {
            l: vec![0; size],
            r: vec![0; size],
            min: vec![0; size],
            max: vec![0; size],
            lazy: vec![0; size],
        };
        st.build(1, 0, n);
        st
    }

    #[inline]
    fn modify(&mut self, u: usize, l: usize, r: usize, v: i32) {
        unsafe {
            if *self.l.get_unchecked(u) >= l && *self.r.get_unchecked(u) <= r {
                self.apply(u, v);
                return;
            }

            self.push_down(u);

            let mid = (*self.l.get_unchecked(u) + *self.r.get_unchecked(u)) >> 1;
            if l <= mid {
                self.modify(u << 1, l, r, v);
            }

            if r > mid {
                self.modify(u << 1 | 1, l, r, v);
            }

            self.push_up(u);
        }
    }

    #[inline]
    fn query(&mut self, u: usize, target: i32) -> usize {
        unsafe {
            if *self.l.get_unchecked(u) == *self.r.get_unchecked(u) {
                return *self.l.get_unchecked(u);
            }

            self.push_down(u);

            let lc = u << 1;
            let rc = u << 1 | 1;
            
            let lc_min = *self.min.get_unchecked(lc);
            let lc_max = *self.max.get_unchecked(lc);
            
            if lc_min <= target && target <= lc_max {
                return self.query(lc, target);
            }

            self.query(rc, target)
        }
    }

    fn build(&mut self, u: usize, l: usize, r: usize) {
        unsafe {
            *self.l.get_unchecked_mut(u) = l;
            *self.r.get_unchecked_mut(u) = r;

            if l == r {
                return;
            }

            let mid = (l + r) >> 1;

            self.build(u << 1, l, mid);
            self.build(u << 1 | 1, mid + 1, r);
        }
    }

    #[inline(always)]
    unsafe fn apply(&mut self, u: usize, v: i32) {
        *self.min.get_unchecked_mut(u) += v;
        *self.max.get_unchecked_mut(u) += v;
        *self.lazy.get_unchecked_mut(u) += v;
    }

    #[inline(always)]
    unsafe fn push_up(&mut self, u: usize) {
        let lc = u << 1;
        let rc = u << 1 | 1;
        
        *self.min.get_unchecked_mut(u) = 
            (*self.min.get_unchecked(lc)).min(*self.min.get_unchecked(rc));
        *self.max.get_unchecked_mut(u) = 
            (*self.max.get_unchecked(lc)).max(*self.max.get_unchecked(rc));
    }

    #[inline(always)]
    unsafe fn push_down(&mut self, u: usize) {
        let lazy_val = *self.lazy.get_unchecked(u);
        if lazy_val != 0 {
            self.apply(u << 1, lazy_val);
            self.apply(u << 1 | 1, lazy_val);
            *self.lazy.get_unchecked_mut(u) = 0;
        }
    }
}
