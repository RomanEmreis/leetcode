/*
  2916. Subarrays Distinct Element Sum of Squares II
  
  You are given a 0-indexed integer array nums.
  The distinct count of a subarray of nums is defined as:
      Let nums[i..j] be a subarray of nums consisting of all the indices from i to j such that 0 <= i <= j < nums.length. 
  
  Then the number of distinct values in nums[i..j] is called the distinct count of nums[i..j].
  
  Return the sum of the squares of distinct counts of all subarrays of nums.
  
  Since the answer may be very large, return it modulo 109 + 7.
  A subarray is a contiguous non-empty sequence of elements within an array.
  
  Example 1:
  Input: nums = [1,2,1]
  Output: 15
  Explanation: Six possible subarrays are:
  [1]: 1 distinct value
  [2]: 1 distinct value
  [1]: 1 distinct value
  [1,2]: 2 distinct values
  [2,1]: 2 distinct values
  [1,2,1]: 2 distinct values
  The sum of the squares of the distinct counts in all subarrays is equal to 12 + 12 + 12 + 22 + 22 + 22 = 15.
  
  Example 2:
  Input: nums = [2,2]
  Output: 3
  Explanation: Three possible subarrays are:
  [2]: 1 distinct value
  [2]: 1 distinct value
  [2,2]: 1 distinct value
  The sum of the squares of the distinct counts in all subarrays is equal to 12 + 12 + 12 = 3.
*/
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = nums.len();
        let max = *nums.iter().max().unwrap() as usize;
        let mut last = vec![0usize; max + 1];
        let mut bit = RangeBit::new(n);

        let mut total = 0i64;
        let mut cur = 0i64;
        let mut res = 0i64;

        for (idx, &x) in nums.iter().enumerate() {
            let r = idx + 1;
            let l = last[x as usize] + 1;
            let len = (r - l + 1) as i64;

            let s = total - bit.prefix(l - 1);
            cur = (cur + 2 * s + len) % MOD;
            res += cur;

            bit.range_add(l, r);
            total += len;
            last[x as usize] = r;
        }

        (res % MOD) as i32
    }
}

struct RangeBit {
    t: Vec<[i64; 2]>,
}

impl RangeBit {
    fn new(n: usize) -> Self {
        Self { t: vec![[0; 2]; n + 1] }
    }

    #[inline]
    fn upd(&mut self, mut i: usize, v1: i64, v2: i64) {
        while i < self.t.len() {
            self.t[i][0] += v1;
            self.t[i][1] += v2;
            i += i & i.wrapping_neg();
        }
    }

    #[inline]
    fn range_add(&mut self, l: usize, r: usize) {
        self.upd(l, 1, (l - 1) as i64);
        self.upd(r + 1, -1, -(r as i64));
    }

    #[inline]
    fn prefix(&self, i: usize) -> i64 {
        let (mut a, mut b, mut j) = (0i64, 0i64, i);
        while j > 0 {
            a += self.t[j][0];
            b += self.t[j][1];
            j &= j - 1;
        }
        a * i as i64 - b
    }
}
