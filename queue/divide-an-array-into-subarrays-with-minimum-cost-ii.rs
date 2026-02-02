/*
  3013. Divide an Array Into Subarrays With Minimum Cost II
  
  You are given a 0-indexed array of integers nums of length n, and two positive integers k and dist.
  The cost of an array is the value of its first element. For example, the cost of [1,2,3] is 1 while the cost of [3,4,1] is 3.
  You need to divide nums into k disjoint contiguous , such that the difference between the starting index of the second subarray and the starting index of the kth subarray 
  should be less than or equal to dist. In other words, if you divide nums into the subarrays nums[0..(i1 - 1)], nums[i1..(i2 - 1)], ..., nums[ik-1..(n - 1)], then ik-1 - i1 <= dist.
  
  Return the minimum possible sum of the cost of these subarrays.
  
  Example 1:
  Input: nums = [1,3,2,6,4,2], k = 3, dist = 3
  Output: 5
  Explanation: The best possible way to divide nums into 3 subarrays is: [1,3], [2,6,4], and [2]. This choice is valid because ik-1 - i1 is 5 - 2 = 3 
  which is equal to dist. The total cost is nums[0] + nums[2] + nums[5] which is 1 + 2 + 2 = 5.
  It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 5.
  
  Example 2:
  Input: nums = [10,1,2,2,2,1], k = 4, dist = 3
  Output: 15
  Explanation: The best possible way to divide nums into 4 subarrays is: [10], [1], [2], and [2,2,1]. This choice is valid because ik-1 - i1 is 3 - 1 = 2 
  which is less than dist. The total cost is nums[0] + nums[1] + nums[2] + nums[3] which is 10 + 1 + 2 + 2 = 15.
  The division [10], [1], [2,2,2], and [1] is not valid, because the difference between ik-1 and i1 is 5 - 1 = 4, which is greater than dist.
  It can be shown that there is no possible way to divide nums into 4 subarrays at a cost lower than 15.
  
  Example 3:
  Input: nums = [10,8,18,9], k = 3, dist = 1
  Output: 36
  Explanation: The best possible way to divide nums into 4 subarrays is: [10], [8], and [18,9]. This choice is valid because ik-1 - i1 is 2 - 1 = 1
  which is equal to dist.The total cost is nums[0] + nums[1] + nums[2] which is 10 + 8 + 18 = 36.
  The division [10], [8,18], and [9] is not valid, because the difference between ik-1 and i1 is 3 - 1 = 2, which is greater than dist.
  It can be shown that there is no possible way to divide nums into 3 subarrays at a cost lower than 36.
*/
use std::cmp::Ordering;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;
        let m = k - 2; // size of small set
        // constraints guarantee k>=3 so m>=1

        let nums64: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();

        // state per index
        let mut active = vec![false; n];
        let mut side = vec![Side::None; n];

        let mut small: Heap<false> = Heap::new(); // max-heap
        let mut large: Heap<true>  = Heap::new(); // min-heap
        let mut sum_small: i64 = 0;

        let mut small_sz: usize = 0;
        let mut large_sz: usize = 0;

        #[inline(always)]
        fn prune<const IS_MIN: bool>(
            heap: &mut Heap<IS_MIN>,
            active: &[bool],
            side: &[Side],
            want: Side,
        ) {
            while let Some(top) = heap.peek() {
                let idx = top.idx;
                if idx >= active.len() || !active[idx] || side[idx] != want {
                    heap.pop();
                } else {
                    break;
                }
            }
        }

        #[inline(always)]
        fn move_large_to_small(
            nums: &[i64],
            active: &[bool],
            side: &mut [Side],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
        ) {
            prune(large, active, side, Side::Large);
            let x = large.pop().unwrap();
            *large_sz -= 1;
            side[x.idx] = Side::Small;
            small.push(x);
            *small_sz += 1;
            *sum_small += x.v;
        }

        #[inline(always)]
        fn move_small_to_large(
            nums: &[i64],
            active: &[bool],
            side: &mut [Side],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
        ) {
            prune(small, active, side, Side::Small);
            let x = small.pop().unwrap();
            *small_sz -= 1;
            *sum_small -= x.v;
            side[x.idx] = Side::Large;
            large.push(x);
            *large_sz += 1;
        }

        #[inline(always)]
        fn rebalance(
            nums: &[i64],
            active: &[bool],
            side: &mut [Side],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            while *small_sz < m && *large_sz > 0 {
                move_large_to_small(nums, active, side, large, small, sum_small, large_sz, small_sz);
            }
            while *small_sz > m {
                move_small_to_large(nums, active, side, large, small, sum_small, large_sz, small_sz);
            }

            // Fix ordering: max(small) <= min(large)
            prune(small, active, side, Side::Small);
            prune(large, active, side, Side::Large);

            if *small_sz > 0 && *large_sz > 0 {
                let a = small.peek().unwrap(); // max of small
                let b = large.peek().unwrap(); // min of large
                if a.v > b.v || (a.v == b.v && a.idx > b.idx) {
                    // swap tops
                    let a = small.pop().unwrap();
                    let b = large.pop().unwrap();
                    *sum_small -= a.v;
                    *sum_small += b.v;

                    side[a.idx] = Side::Large;
                    side[b.idx] = Side::Small;

                    small.push(b);
                    large.push(a);
                }
            }
        }

        #[inline(always)]
        fn add_idx(
            nums: &[i64],
            idx: usize,
            active: &mut [bool],
            side: &mut [Side],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            active[idx] = true;

            // Decide side quickly using boundary = min(large)
            if *large_sz > 0 {
                prune(large, active, side, Side::Large);
                let b = large.peek().unwrap();
                if nums[idx] >= b.v {
                    side[idx] = Side::Large;
                    large.push(Node { v: nums[idx], idx });
                    *large_sz += 1;
                } else {
                    side[idx] = Side::Small;
                    small.push(Node { v: nums[idx], idx });
                    *small_sz += 1;
                    *sum_small += nums[idx];
                }
            } else {
                // if large empty, put into small first
                side[idx] = Side::Small;
                small.push(Node { v: nums[idx], idx });
                *small_sz += 1;
                *sum_small += nums[idx];
            }

            rebalance(nums, active, side, large, small, sum_small, large_sz, small_sz, m);
        }

        #[inline(always)]
        fn remove_idx(
            nums: &[i64],
            idx: usize,
            active: &mut [bool],
            side: &mut [Side],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            if !active[idx] { return; }
            active[idx] = false;

            match side[idx] {
                Side::Small => {
                    *small_sz -= 1;
                    *sum_small -= nums[idx];
                }
                Side::Large => {
                    *large_sz -= 1;
                }
                Side::None => {}
            }
            side[idx] = Side::None;

            rebalance(nums, active, side, large, small, sum_small, large_sz, small_sz, m);
        }

        // We iterate i1 (start of 2nd subarray) from 1..=max_i1
        // Need enough elements after i1 to pick k-2 starts: i1 <= n-k+1
        let max_i1 = n - k + 1;

        // Initial window for i1=1: indices [2 .. min(n-1, 1+dist)]
        let mut right = (1 + dist).min(n - 1);
        for idx in 2..=right {
            add_idx(&nums64, idx, &mut active, &mut side, &mut large, &mut small,
                    &mut sum_small, &mut large_sz, &mut small_sz, m);
        }

        let mut ans: i64 = i64::MAX;

        for i1 in 1..=max_i1 {
            // cost = nums[0] + nums[i1] + sum_small
            let cost = nums64[0] + nums64[i1] + sum_small;
            if cost < ans { ans = cost; }

            // Slide to next i1:
            // window loses idx = i1+1, gains new indices if right expands to (i1+1 + dist)
            let out = i1 + 1;
            remove_idx(&nums64, out, &mut active, &mut side, &mut large, &mut small,
                       &mut sum_small, &mut large_sz, &mut small_sz, m);

            let next_i1 = i1 + 1;
            let new_right = (next_i1 + dist).min(n - 1);
            if new_right > right {
                for idx in (right + 1)..=new_right {
                    add_idx(&nums64, idx, &mut active, &mut side, &mut large, &mut small,
                            &mut sum_small, &mut large_sz, &mut small_sz, m);
                }
                right = new_right;
            }
        }

        ans
    }
}

#[derive(Copy, Clone)]
struct Node {
    v: i64,
    idx: usize,
}

#[inline(always)]
fn less_max(a: Node, b: Node) -> bool {
    // max-heap: "a has lower priority than b"?
    // Want bigger value first; tie by bigger idx first (stable-ish).
    if a.v != b.v { a.v < b.v } else { a.idx < b.idx }
}

#[inline(always)]
fn less_min(a: Node, b: Node) -> bool {
    // min-heap: smaller value first; tie by smaller idx first
    if a.v != b.v { a.v > b.v } else { a.idx > b.idx }
}

struct Heap<const IS_MIN: bool> {
    a: Vec<Node>,
}

impl<const IS_MIN: bool> Heap<IS_MIN> {
    #[inline(always)]
    fn new() -> Self { Self { a: Vec::new() } }

    #[inline(always)]
    fn len(&self) -> usize { self.a.len() }

    #[inline(always)]
    fn is_empty(&self) -> bool { self.a.is_empty() }

    #[inline(always)]
    fn better(x: Node, y: Node) -> bool {
        // "x should be above y"
        if IS_MIN {
            // x better if it is smaller
            if x.v != y.v { x.v < y.v } else { x.idx < y.idx }
        } else {
            // max-heap
            if x.v != y.v { x.v > y.v } else { x.idx > y.idx }
        }
    }

    #[inline(always)]
    fn push(&mut self, x: Node) {
        self.a.push(x);
        unsafe {
            let mut i = self.a.len() - 1;
            while i > 0 {
                let p = (i - 1) >> 1;
                let cur = *self.a.get_unchecked(i);
                let par = *self.a.get_unchecked(p);
                if Self::better(cur, par) {
                    let ptr = self.a.as_mut_ptr();
                    std::ptr::swap(ptr.add(i), ptr.add(p));
                    i = p;
                } else {
                    break;
                }
            }
        }
    }

    #[inline(always)]
    fn peek(&self) -> Option<Node> {
        self.a.get(0).copied()
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<Node> {
        let n = self.a.len();
        if n == 0 { return None; }
        if n == 1 { return self.a.pop(); }
        unsafe {
            let ptr = self.a.as_mut_ptr();
            std::ptr::swap(ptr, ptr.add(n - 1));
        }
        let res = self.a.pop();
        self.sift_down(0);
        res
    }

    #[inline(always)]
    fn sift_down(&mut self, mut i: usize) {
        unsafe {
            let n = self.a.len();
            let ptr = self.a.as_mut_ptr();
            loop {
                let l = i * 2 + 1;
                if l >= n { break; }
                let r = l + 1;

                let mut best = l;
                if r < n {
                    let nl = *ptr.add(l);
                    let nr = *ptr.add(r);
                    if Self::better(nr, nl) {
                        best = r;
                    }
                }

                let cur = *ptr.add(i);
                let child = *ptr.add(best);
                if Self::better(child, cur) {
                    std::ptr::swap(ptr.add(i), ptr.add(best));
                    i = best;
                } else {
                    break;
                }
            }
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Side {
    None = 0,
    Large = 1,
    Small = 2,
}
