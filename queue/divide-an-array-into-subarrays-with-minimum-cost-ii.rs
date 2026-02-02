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
#[inline(always)]
fn pack(val: u32, idx: u32) -> u64 {
    ((val as u64) << 32) | (idx as u64)
}
#[inline(always)]
fn val_of(x: u64) -> u32 { (x >> 32) as u32 }
#[inline(always)]
fn idx_of(x: u64) -> usize { (x as u32) as usize }

// state byte:
// 0 = inactive
// 1 = active large
// 2 = active small
const INACTIVE: u8 = 0;
const LARGE: u8 = 1;
const SMALL: u8 = 2;

struct Heap<const IS_MIN: bool> {
    a: Vec<u64>, // packed(val, idx)
}

impl<const IS_MIN: bool> Heap<IS_MIN> {
    #[inline(always)]
    fn new() -> Self { Self { a: Vec::new() } }
    #[inline(always)]
    fn len(&self) -> usize { self.a.len() }
    #[inline(always)]
    fn is_empty(&self) -> bool { self.a.is_empty() }
    #[inline(always)]
    fn peek(&self) -> Option<u64> { self.a.get(0).copied() }

    #[inline(always)]
    fn better(x: u64, y: u64) -> bool {
        // x should be above y
        let xv = val_of(x);
        let yv = val_of(y);
        let xi = x as u32;
        let yi = y as u32;

        if IS_MIN {
            // smaller value first, tie smaller idx
            if xv != yv { xv < yv } else { xi < yi }
        } else {
            // larger value first, tie larger idx
            if xv != yv { xv > yv } else { xi > yi }
        }
    }

    #[inline(always)]
    fn push(&mut self, x: u64) {
        self.a.push(x);
        unsafe {
            let mut i = self.a.len() - 1;
            let ptr = self.a.as_mut_ptr();
            while i > 0 {
                let p = (i - 1) >> 1;
                let cur = *ptr.add(i);
                let par = *ptr.add(p);
                if Self::better(cur, par) {
                    core::ptr::swap(ptr.add(i), ptr.add(p));
                    i = p;
                } else {
                    break;
                }
            }
        }
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<u64> {
        let n = self.a.len();
        if n == 0 { return None; }
        if n == 1 { return self.a.pop(); }
        unsafe {
            let ptr = self.a.as_mut_ptr();
            core::ptr::swap(ptr, ptr.add(n - 1));
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
                    let xl = *ptr.add(l);
                    let xr = *ptr.add(r);
                    if Self::better(xr, xl) {
                        best = r;
                    }
                }

                let cur = *ptr.add(i);
                let child = *ptr.add(best);
                if Self::better(child, cur) {
                    core::ptr::swap(ptr.add(i), ptr.add(best));
                    i = best;
                } else {
                    break;
                }
            }
        }
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;
        let m = k - 2; // small size

        // combined state: 0/1/2
        let mut st = vec![INACTIVE; n];

        let mut small: Heap<false> = Heap::new(); // max-heap of SMALL
        let mut large: Heap<true>  = Heap::new(); // min-heap of LARGE

        let mut sum_small: i64 = 0;
        let mut small_sz: usize = 0;
        let mut large_sz: usize = 0;

        #[inline(always)]
        fn prune<const IS_MIN: bool>(
            heap: &mut Heap<IS_MIN>,
            st: &[u8],
            want: u8,
        ) {
            while let Some(top) = heap.peek() {
                let idx = idx_of(top);
                if idx >= st.len() || st[idx] != want {
                    heap.pop();
                } else {
                    break;
                }
            }
        }

        #[inline(always)]
        fn move_large_to_small(
            nums: &[i32],
            st: &mut [u8],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
        ) {
            prune(large, st, LARGE);
            let x = large.pop().unwrap();
            *large_sz -= 1;
            let idx = idx_of(x);
            st[idx] = SMALL;
            small.push(x);
            *small_sz += 1;
            *sum_small += nums[idx] as i64;
        }

        #[inline(always)]
        fn move_small_to_large(
            nums: &[i32],
            st: &mut [u8],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
        ) {
            prune(small, st, SMALL);
            let x = small.pop().unwrap();
            *small_sz -= 1;
            let idx = idx_of(x);
            *sum_small -= nums[idx] as i64;
            st[idx] = LARGE;
            large.push(x);
            *large_sz += 1;
        }

        #[inline(always)]
        fn rebalance(
            nums: &[i32],
            st: &mut [u8],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            while *small_sz < m && *large_sz > 0 {
                move_large_to_small(nums, st, large, small, sum_small, large_sz, small_sz);
            }
            while *small_sz > m {
                move_small_to_large(nums, st, large, small, sum_small, large_sz, small_sz);
            }

            // optional: enforce order by swapping tops if needed
            prune(small, st, SMALL);
            prune(large, st, LARGE);
            if *small_sz > 0 && *large_sz > 0 {
                let a = small.peek().unwrap(); // max(small)
                let b = large.peek().unwrap(); // min(large)
                let av = val_of(a);
                let bv = val_of(b);
                let ai = a as u32;
                let bi = b as u32;
                if av > bv || (av == bv && ai > bi) {
                    let a = small.pop().unwrap();
                    let b = large.pop().unwrap();
                    let aidx = idx_of(a);
                    let bidx = idx_of(b);

                    *sum_small -= nums[aidx] as i64;
                    *sum_small += nums[bidx] as i64;

                    st[aidx] = LARGE;
                    st[bidx] = SMALL;

                    small.push(b);
                    large.push(a);
                }
            }
        }

        #[inline(always)]
        fn add_idx(
            nums: &[i32],
            idx: usize,
            st: &mut [u8],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            let key = pack(nums[idx] as u32, idx as u32);

            if *large_sz > 0 {
                prune(large, st, LARGE);
                let b = large.peek().unwrap();
                let bv = val_of(b);

                if (nums[idx] as u32) >= bv {
                    st[idx] = LARGE;
                    large.push(key);
                    *large_sz += 1;
                } else {
                    st[idx] = SMALL;
                    small.push(key);
                    *small_sz += 1;
                    *sum_small += nums[idx] as i64;
                }
            } else {
                st[idx] = SMALL;
                small.push(key);
                *small_sz += 1;
                *sum_small += nums[idx] as i64;
            }

            rebalance(nums, st, large, small, sum_small, large_sz, small_sz, m);
        }

        #[inline(always)]
        fn remove_idx(
            nums: &[i32],
            idx: usize,
            st: &mut [u8],
            large: &mut Heap<true>,
            small: &mut Heap<false>,
            sum_small: &mut i64,
            large_sz: &mut usize,
            small_sz: &mut usize,
            m: usize,
        ) {
            match st[idx] {
                SMALL => {
                    *small_sz -= 1;
                    *sum_small -= nums[idx] as i64;
                }
                LARGE => {
                    *large_sz -= 1;
                }
                _ => {}
            }
            st[idx] = INACTIVE;
            rebalance(nums, st, large, small, sum_small, large_sz, small_sz, m);
        }

        // i1 range
        let max_i1 = n - k + 1;

        // initial window for i1=1: [2..=min(n-1,1+dist)]
        let mut right = (1 + dist).min(n - 1);
        for idx in 2..=right {
            add_idx(&nums, idx, &mut st, &mut large, &mut small,
                    &mut sum_small, &mut large_sz, &mut small_sz, m);
        }

        let mut ans = i64::MAX;

        for i1 in 1..=max_i1 {
            let cost = nums[0] as i64 + nums[i1] as i64 + sum_small;
            if cost < ans { ans = cost; }

            // slide
            let out = i1 + 1;
            remove_idx(&nums, out, &mut st, &mut large, &mut small,
                       &mut sum_small, &mut large_sz, &mut small_sz, m);

            let next_i1 = i1 + 1;
            let new_right = (next_i1 + dist).min(n - 1);
            if new_right > right {
                for idx in (right + 1)..=new_right {
                    add_idx(&nums, idx, &mut st, &mut large, &mut small,
                            &mut sum_small, &mut large_sz, &mut small_sz, m);
                }
                right = new_right;
            }
        }

        ans
    }
}

