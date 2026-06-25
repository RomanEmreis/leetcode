/*
  3737. Count Subarrays With Majority Element I
  
  You are given an integer array nums and an integer target.
  Return the number of of nums in which target is the majority element.
  
  The majority element of a subarray is the element that appears strictly more than half of the times in that subarray.
  
  Example 1:
  Input: nums = [1,2,2,3], target = 2
  Output: 5
  Explanation:
  Valid subarrays with target = 2 as the majority element:
      nums[1..1] = [2]
      nums[2..2] = [2]
      nums[1..2] = [2,2]
      nums[0..2] = [1,2,2]
      nums[1..3] = [2,2,3]
  
  So there are 5 such subarrays.
  
  Example 2:
  Input: nums = [1,1,1,1], target = 1
  Output: 10
  Explanation:
  ​​​​​​​All 10 subarrays have 1 as the majority element.
  
  Example 3:
  Input: nums = [1,2,3], target = 4
  Output: 0
  Explanation:
  target = 4 does not appear in nums at all. Therefore, there cannot be any subarray where 4 is the majority element. Hence the answer is 0.
*/
impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut bit = Bit::new(2 * n + 1);
        bit.add(n, 1);

        let mut prefix = n as i32;
        let mut res = 0;

        for &x in &nums {
            prefix += if x == target { 1 } else { -1 };
            let idx = prefix as usize;

            res += bit.query(idx - 1);
            bit.add(idx, 1);
        }
        res
    }
}

struct Bit {
    tree: Vec<i32>,
}

impl Bit {
    fn new(size: usize) -> Self {
        Bit { tree: vec![0; size + 1] }
    }

    fn add(&mut self, pos: usize, delta: i32) {
        let mut i = pos + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn query(&self, pos: usize) -> i32 {
        let mut i = pos.wrapping_add(1);
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}
