/*
  3600. Maximize Spanning Tree Stability with Upgrades
  
  You are given an integer n, representing n nodes numbered from 0 to n - 1 and a list of edges, where edges[i] = [ui, vi, si, musti]:
      ui and vi indicates an undirected edge between nodes ui and vi.
      si is the strength of the edge.
      musti is an integer (0 or 1). If musti == 1, the edge must be included in the spanning tree. These edges cannot be upgraded.
  
  You are also given an integer k, the maximum number of upgrades you can perform. Each upgrade doubles the strength of an edge, and each eligible edge (with musti == 0) can be upgraded at most once.
  The stability of a spanning tree is defined as the minimum strength score among all edges included in it.
  
  Return the maximum possible stability of any valid spanning tree. If it is impossible to connect all nodes, return -1.
  
  Note: A spanning tree of a graph with n nodes is a subset of the edges that connects all nodes together (i.e. the graph is connected) without forming any cycles, and uses exactly n - 1 edges.
  
  Example 1:
  Input: n = 3, edges = [[0,1,2,1],[1,2,3,0]], k = 1
  Output: 2
  Explanation:
      Edge [0,1] with strength = 2 must be included in the spanning tree.
      Edge [1,2] is optional and can be upgraded from 3 to 6 using one upgrade.
      The resulting spanning tree includes these two edges with strengths 2 and 6.
      The minimum strength in the spanning tree is 2, which is the maximum possible stability.
  
  Example 2:
  Input: n = 3, edges = [[0,1,4,0],[1,2,3,0],[0,2,1,0]], k = 2
  Output: 6
  Explanation:
      Since all edges are optional and up to k = 2 upgrades are allowed.
      Upgrade edges [0,1] from 4 to 8 and [1,2] from 3 to 6.
      The resulting spanning tree includes these two edges with strengths 8 and 6.
      The minimum strength in the tree is 6, which is the maximum possible stability.
  
  Example 3:
  Input: n = 3, edges = [[0,1,1,1],[1,2,1,1],[2,0,1,1]], k = 0
  Output: -1
  Explanation:
      All edges are mandatory and form a cycle, which violates the spanning tree property of acyclicity. Thus, the answer is -1.
*/
struct UnionFind {
    p: Vec<u32>,
    sz: Vec<u32>,
    pub cnt: u32,
}

impl UnionFind {
    fn new(n: u32) -> Self {
        Self {
            p: (0..n).collect(),
            sz: vec![1; n as usize],
            cnt: n,
        }
    }

    #[inline]
    fn find(&mut self, mut x: u32) -> u32 {
        while self.p[x as usize] != x {
            let px = self.p[x as usize] as usize;
            self.p[x as usize] = self.p[px];
            x = self.p[x as usize];
        }
        x
    }

    #[inline]
    fn union(&mut self, a: u32, b: u32) -> bool {
        let (pa, pb) = (self.find(a), self.find(b));
        if pa == pb {
            return false;
        }
        let (a, b) = (pa as usize, pb as usize);
        if self.sz[a] < self.sz[b] {
            self.p[a] = pb as u32;
            self.sz[b] += self.sz[a];
        } else {
            self.p[b] = pa as u32;
            self.sz[a] += self.sz[b];
        }
        self.cnt -= 1;
        true
    }
}

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as u32;
        let k = k as u32;

        let mut required: Vec<[u32; 3]> = Vec::new();
        let mut optional: Vec<[u32; 3]> = Vec::new();

        for e in &edges {
            let (u, v, w, req) = (e[0] as u32, e[1] as u32, e[2] as u32, e[3]);
            if req == 1 {
                required.push([u, v, w]);
            } else {
                optional.push([u, v, w]);
            }
        }

        let mut uf = UnionFind::new(n);
        for &[u, v, _] in &required {
            if !uf.union(u, v) {
                return -1;
            }
        }

        let mut uf_full = UnionFind::new(n);
        for &[u, v, _] in required.iter().chain(optional.iter()) {
            uf_full.union(u, v);
        }
        if uf_full.cnt > 1 {
            return -1;
        }

        optional.sort_unstable_by_key(|e| e[2]);

        let upper = if !required.is_empty() {
            required.iter().map(|e| e[2]).min().unwrap()
        } else {
            optional.last().map_or(0, |e| e[2]) * 2
        };

        let (mut l, mut r) = (1u32, upper);
        while l < r {
            let mid = (l + r + 1) / 2;
            if check(mid, n, k, &required, &optional) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        l as i32
    }
}

#[inline]
fn check(lim: u32, n: u32, k: u32, required: &[[u32; 3]], optional: &[[u32; 3]]) -> bool {
    let mut uf = UnionFind::new(n);

    for &[u, v, _] in required {
        uf.union(u, v);
    }

    let split = optional.partition_point(|e| e[2] < lim);
    for &[u, v, _] in &optional[split..] {
        uf.union(u, v);
    }

    if uf.cnt == 1 {
        return true;
    }

    let half_split = optional.partition_point(|e| e[2] * 2 < lim);
    let mut rem = k;
    for &[u, v, _] in optional[half_split..split].iter().rev() {
        if rem == 0 {
            break;
        }
        if uf.union(u, v) {
            rem -= 1;
        }
    }

    uf.cnt == 1
}
