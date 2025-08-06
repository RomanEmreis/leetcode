/*
  3479. Fruits Into Baskets III
  
  You are given two arrays of integers, fruits and baskets, each of length n, where fruits[i] represents the quantity of the ith type of fruit, and baskets[j] represents the capacity of the jth basket.
  From left to right, place the fruits according to these rules:
    Each fruit type must be placed in the leftmost available basket with a capacity greater than or equal to the quantity of that fruit type.
    Each basket can hold only one type of fruit.
    If a fruit type cannot be placed in any basket, it remains unplaced.
  
  Return the number of fruit types that remain unplaced after all possible allocations are made.
  
  Example 1:
  Input: fruits = [4,2,5], baskets = [3,5,4]
  Output: 1
  Explanation:
  fruits[0] = 4 is placed in baskets[1] = 5.
  fruits[1] = 2 is placed in baskets[0] = 3.
  fruits[2] = 5 cannot be placed in baskets[2] = 4.
  Since one fruit type remains unplaced, we return 1.
  
  Example 2:
  Input: fruits = [3,6,1], baskets = [6,4,7]
  Output: 0
  Explanation:
  fruits[0] = 3 is placed in baskets[0] = 6.
  fruits[1] = 6 cannot be placed in baskets[1] = 4 (insufficient capacity) but can be placed in the next available basket, baskets[2] = 7.
  fruits[2] = 1 is placed in baskets[1] = 4.
  Since all fruits are successfully placed, we return 0.
*/
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut tree = SegmentTree::new(baskets);
        fruits.iter().for_each(|&f| {
            if tree.query(f) == -1 {
                result += 1;
            }
        });
        result
    }
}

struct SegmentTree {
    cnt: usize,
    tree: Vec<i32>
}

impl SegmentTree {
    fn new(nums: Vec<i32>) -> Self {
        let cnt = nums.len();
        let mut st = Self {
            tree: vec![0; cnt * 4],
            cnt,
        };
        st.build(&nums, 0, 0, cnt - 1);
        st
    }

    fn build(&mut self, nums: &Vec<i32>, idx: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.tree[idx] = nums[lo];
            return;
        }
        let mid = (lo + hi) / 2;
        self.build(nums, 2 * idx + 1, lo, mid);
        self.build(nums, 2 * idx + 2, mid + 1, hi);
        self.tree[idx] = self.tree[2 * idx + 1].max(self.tree[2 * idx + 2]);
    }

    fn query(&mut self, target: i32) -> i32 {
        self.query_first(target, 0, 0, self.cnt - 1)
    }

    fn query_first(&mut self, target: i32, idx: usize, lo: usize, hi: usize) -> i32 {
        if self.tree[idx] < target {
            return -1;
        }
        if lo == hi {
            self.update(lo, -1, 0, 0, self.cnt - 1);
            return lo as i32;
        }
        let mid = (lo + hi) / 2;
        let left = self.tree[2 * idx + 1];
        if left >= target {
            self.query_first(target, 2 * idx + 1, lo, mid)
        } else {
            self.query_first(target, 2 * idx + 2, mid+ 1, hi)
        }
    }

    fn update(&mut self, i: usize, val: i32, idx: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.tree[idx] = val;
            return;
        }
        let mid = (lo + hi) / 2;
        if i <= mid {
            self.update(i, val, 2 * idx + 1, lo, mid);
        } else {
            self.update(i, val, 2 * idx + 2, mid + 1, hi);
        }
        self.tree[idx] = self.tree[2 * idx + 1].max(self.tree[2 * idx + 2]);
    }
}
