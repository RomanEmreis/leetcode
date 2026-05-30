/*
  3161. Block Placement Queries
  
  There exists an infinite number line, with its origin at 0 and extending towards the positive x-axis.
  You are given a 2D array queries, which contains two types of queries:
      For a query of type 1, queries[i] = [1, x]. Build an obstacle at distance x from the origin. It is guaranteed that there is no obstacle at distance x when the query is asked.
      For a query of type 2, queries[i] = [2, x, sz]. Check if it is possible to place a block of size sz anywhere in the range [0, x] on the line, 
    such that the block entirely lies in the range [0, x]. A block cannot be placed if it intersects with any obstacle, but it may touch it. Note that you do not actually place the block. Queries are separate.
  
  Return a boolean array results, where results[i] is true if you can place the block specified in the ith query of type 2, and false otherwise.
  
  Example 1:
  Input: queries = [[1,2],[2,3,3],[2,3,1],[2,2,2]]
  Output: [false,true,true]
  Explanation:
  For query 0, place an obstacle at x = 2. A block of size at most 2 can be placed before x = 3.
  
  Example 2:
  Input: queries = [[1,7],[2,7,6],[1,2],[2,7,5],[2,7,6]]
  Output: [true,true,false]
  Explanation:
      Place an obstacle at x = 7 for query 0. A block of size at most 7 can be placed before x = 7.
      Place an obstacle at x = 2 for query 2. Now, a block of size at most 5 can be placed before x = 7, and a block of size at most 2 before x = 2.
*/
use std::collections::HashMap;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = 50_000.min(queries.len() as i32 * 3);

        let mut coords = vec![0, n];

        for q in &queries {
            if q[0] == 1 {
                coords.push(q[1]);
            }
        }

        coords.sort_unstable();
        coords.dedup();

        let mut index = HashMap::with_capacity(coords.len());
        for (i, &x) in coords.iter().enumerate() {
            index.insert(x, i);
        }

        let mut max_tree = FenwickMax::new(n as usize + 2);
        let mut active_tree = FenwickSum::new(coords.len());

        let m = coords.len();

        let mut prev = vec![usize::MAX; m];
        let mut next = vec![usize::MAX; m];

        for i in 0..m {
            if i > 0 {
                prev[i] = i - 1;
            }

            if i + 1 < m {
                next[i] = i + 1;
            }

            active_tree.add(i + 1, 1);
        }

        for i in 0..m - 1 {
            let x1 = coords[i];
            let x2 = coords[i + 1];
            max_tree.maximize(x2 as usize, x2 - x1);
        }

        let mut ans = Vec::new();

        for q in queries.iter().rev() {
            let typ = q[0];
            let x = q[1];

            if typ == 1 {
                let id = index[&x];

                let p = prev[id];
                let nx = next[id];

                if nx != usize::MAX {
                    max_tree.maximize(coords[nx] as usize, coords[nx] - coords[p]);

                    next[p] = nx;
                    prev[nx] = p;
                }

                active_tree.add(id + 1, -1);
            } else {
                let sz = q[2];

                let upper = upper_bound(&coords, x);
                let count = active_tree.sum(upper);

                let pred_id = active_tree.find_by_order(count) - 1;
                let pred = coords[pred_id];

                ans.push(max_tree.get(pred as usize) >= sz || x - pred >= sz);
            }
        }

        ans.reverse();
        ans
    }
}

fn upper_bound(arr: &[i32], x: i32) -> usize {
    let mut l = 0;
    let mut r = arr.len();

    while l < r {
        let m = l + (r - l) / 2;

        if arr[m] <= x {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l
}

struct FenwickMax {
    tree: Vec<i32>,
}

impl FenwickMax {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn maximize(&mut self, mut i: usize, value: i32) {
        while i < self.tree.len() {
            self.tree[i] = self.tree[i].max(value);
            i += i & (!i + 1);
        }
    }

    fn get(&self, mut i: usize) -> i32 {
        let mut res = 0;

        while i > 0 {
            res = res.max(self.tree[i]);
            i -= i & (!i + 1);
        }

        res
    }
}

struct FenwickSum {
    tree: Vec<i32>,
}

impl FenwickSum {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn sum(&self, mut i: usize) -> i32 {
        let mut res = 0;

        while i > 0 {
            res += self.tree[i];
            i -= i & (!i + 1);
        }

        res
    }

    fn find_by_order(&self, mut k: i32) -> usize {
        let mut idx = 0;
        let mut bit = 1;

        while (bit << 1) < self.tree.len() {
            bit <<= 1;
        }

        while bit != 0 {
            let next = idx + bit;

            if next < self.tree.len() && self.tree[next] < k {
                idx = next;
                k -= self.tree[next];
            }

            bit >>= 1;
        }

        idx + 1
    }
}
