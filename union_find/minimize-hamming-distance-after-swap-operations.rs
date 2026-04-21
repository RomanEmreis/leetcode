/*
  1722. Minimize Hamming Distance After Swap Operations
  
  You are given two integer arrays, source and target, both of length n. You are also given an array allowedSwaps where each allowedSwaps[i] = [ai, bi] indicates 
  that you are allowed to swap the elements at index ai and index bi (0-indexed) of array source. Note that you can swap elements at a specific pair of indices multiple times and in any order.
  
  The Hamming distance of two arrays of the same length, source and target, is the number of positions where the elements are different. 
  Formally, it is the number of indices i for 0 <= i <= n-1 where source[i] != target[i] (0-indexed).
  
  Return the minimum Hamming distance of source and target after performing any amount of swap operations on array source.
  
  Example 1:
  Input: source = [1,2,3,4], target = [2,1,4,5], allowedSwaps = [[0,1],[2,3]]
  Output: 1
  Explanation: source can be transformed the following way:
  - Swap indices 0 and 1: source = [2,1,3,4]
  - Swap indices 2 and 3: source = [2,1,4,3]
  The Hamming distance of source and target is 1 as they differ in 1 position: index 3.
  
  Example 2:
  Input: source = [1,2,3,4], target = [1,3,2,4], allowedSwaps = []
  Output: 2
  Explanation: There are no allowed swaps.
  The Hamming distance of source and target is 2 as they differ in 2 positions: index 1 and index 2.
  
  Example 3:
  Input: source = [5,1,2,4,3], target = [1,5,4,2,3], allowedSwaps = [[0,4],[4,2],[1,3],[1,4]]
  Output: 0
*/
struct UnionFind {
    id: Vec<usize>,
    rank: Vec<u8>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let id = (0..n).collect();
        Self { id, rank: vec![0; n] }
    }

    #[inline(always)]
    fn union(&mut self, u: usize, v: usize) {
        let i = self.find(u);
        let j = self.find(v);
        if i == j {
            return;
        }

        match self.rank[i].cmp(&self.rank[j]) {
            std::cmp::Ordering::Less => self.id[i] = j,
            std::cmp::Ordering::Greater => self.id[j] = i,
            std::cmp::Ordering::Equal => {
                self.id[i] = j;
                self.rank[j] += 1;
            }
        }
    }

    #[inline(always)]
    fn find(&mut self, u: usize) -> usize {
        let mut u = u;
        while self.id[u] != u {
            let p = self.id[u];
            self.id[u] = self.id[p];
            u = p;
        }
        u
    }
}


#[inline(always)]
fn pack(root: usize, value: i32) -> u64 {
    ((root as u64) << 32) | (value as u32 as u64)
}

impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let n = source.len();
        let mut uf = UnionFind::new(n);

        for allowed_swap in allowed_swaps {
            uf.union(
                allowed_swap[0] as usize,
                allowed_swap[1] as usize,
            );
        }

        for i in 0..n {
            let root = uf.find(i);
            uf.id[i] = root;
        }

        let mut a = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);

        for i in 0..n {
            let root = uf.id[i];
            a.push(pack(root, source[i]));
            b.push(pack(root, target[i]));
        }

        a.sort_unstable();
        b.sort_unstable();

        let mut i = 0usize;
        let mut j = 0usize;
        let mut matched = 0i32;

        while i < n && j < n {
            use std::cmp::Ordering::*;
            match a[i].cmp(&b[j]) {
                Less => i += 1,
                Greater => j += 1,
                Equal => {
                    matched += 1;
                    i += 1;
                    j += 1;
                }
            }
        }

        n as i32 - matched
    }
}
