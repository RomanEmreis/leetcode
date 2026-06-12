/*
  3559. Number of Ways to Assign Edge Weights II
  
  There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer array edges of length n - 1, 
  where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
  Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
  The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
  
  You are given a 2D integer array queries. For each queries[i] = [ui, vi], determine the number of ways to assign weights to edges 
  in the path such that the cost of the path between ui and vi is odd.
  
  Return an array answer, where answer[i] is the number of valid assignments for queries[i].
  
  Since the answer may be large, apply modulo 109 + 7 to each answer[i].
  
  Note: For each query, disregard all edges not in the path between node ui and vi.
  
  Example 1:
  Input: edges = [[1,2]], queries = [[1,1],[1,2]]
  Output: [0,1]
  Explanation:
      Query [1,1]: The path from Node 1 to itself consists of no edges, so the cost is 0. Thus, the number of valid assignments is 0.
      Query [1,2]: The path from Node 1 to Node 2 consists of one edge (1 → 2). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.
  
  Example 2:
  Input: edges = [[1,2],[1,3],[3,4],[3,5]], queries = [[1,4],[3,4],[2,5]]
  Output: [2,1,4]
  Explanation:
      Query [1,4]: The path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4). Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.
      Query [3,4]: The path from Node 3 to Node 4 consists of one edge (3 → 4). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.
      Query [2,5]: The path from Node 2 to Node 5 consists of three edges (2 → 1, 1 → 3, and 3 → 5). Assigning (1,2,2), (2,1,2), (2,2,1), or (1,1,1) makes the cost odd. Thus, the number of valid assignments is 4.
*/
const MOD: u32 = 1_000_000_007;
const B: usize = 32;
const BSH: u32 = 5;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;

        let mut start = vec![0u32; n + 1];
        for e in &edges {
            start[e[0] as usize] += 1;
            start[e[1] as usize] += 1;
        }

        let mut acc = 0u32;
        for s in start.iter_mut().skip(1) {
            let deg = *s;
            *s = acc;
            acc += deg;
        }

        let mut adj = vec![0u32; 2 * (n - 1)];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[start[u] as usize] = e[1] as u32;
            start[u] += 1;
            adj[start[v] as usize] = e[0] as u32;
            start[v] += 1;
        }

        let m = 2 * n - 1;
        let mut ed = vec![0u32; m];
        let mut first = vec![0u32; n + 1];
        let mut st_node = vec![0u32; n];
        let mut st_edge = vec![0u32; n];

        st_node[0] = 1;
        st_edge[0] = start[0];

        let (mut sp, mut t) = (0usize, 1usize);
        let mut cur_depth = 0u32;
        loop {
            let u = st_node[sp] as usize;
            let (mut e, end) = (st_edge[sp], start[u]);
            let mut child = 0u32;

            while e < end {
                let v = adj[e as usize];
                e += 1;
                if v != 1 && first[v as usize] == 0 {
                    child = v;
                    break;
                }
            }

            st_edge[sp] = e;

            if child != 0 {
                cur_depth += 1;
                first[child as usize] = t as u32;
                ed[t] = cur_depth;
                t += 1;
                sp += 1;
                st_node[sp] = child;
                st_edge[sp] = start[child as usize - 1];
            } else if sp == 0 {
                break;
            } else {
                sp -= 1;
                cur_depth -= 1;
                ed[t] = cur_depth;
                t += 1;
            }
        }
        drop(st_node);
        drop(st_edge);

        let nb = (m + B - 1) / B;
        let lev = nb.ilog2() as usize + 1;

        let mut off = vec![0usize; lev + 1];
        for k in 0..lev {
            off[k + 1] = off[k] + (nb - (1 << k) + 1);
        }
        let mut tab = vec![0u32; off[lev]];
        for (c, chunk) in tab[..nb].iter_mut().zip(ed.chunks(B)) {
            *c = chunk.iter().fold(u32::MAX, |a, &b| a.min(b));
        }
        
        for k in 1..lev {
            let half = 1usize << (k - 1);
            let len = nb - (1 << k) + 1;
            let (lo, hi) = tab.split_at_mut(off[k]);
            let prev = &lo[off[k - 1]..];
            for ((c, &a), &b) in hi[..len].iter_mut().zip(prev).zip(&prev[half..]) {
                *c = a.min(b);
            }
        }

        let mut pow2 = vec![0u32; 2 * n];
        pow2[0] = 1;
        for i in 1..2 * n {
            let x = pow2[i - 1] << 1;
            pow2[i] = if x >= MOD { x - MOD } else { x };
        }

        let range_min = |l: usize, r: usize| -> u32 {
            let (bl, br) = (l >> BSH, r >> BSH);
            if bl == br {
                return ed[l..=r].iter().fold(u32::MAX, |a, &b| a.min(b));
            }
            let mut res = ed[l..(bl + 1) << BSH].iter().fold(u32::MAX, |a, &b| a.min(b));
            res = ed[br << BSH..=r].iter().fold(res, |a, &b| a.min(b));
            if bl + 1 < br {
                let (x, y) = (bl + 1, br - 1);
                let k = (y - x + 1).ilog2() as usize;
                let row = &tab[off[k]..];
                res = res.min(row[x]).min(row[y + 1 - (1 << k)]);
            }
            res
        };

        queries
            .iter()
            .map(|q| {
                let (u, v) = (q[0] as usize, q[1] as usize);
                if u == v {
                    return 0;
                }
                let (mut l, mut r) = (first[u], first[v]);
                if l > r {
                    std::mem::swap(&mut l, &mut r);
                }
                let (l, r) = (l as usize, r as usize);
                let dl = range_min(l, r);
                
                pow2[(ed[l] + ed[r] - 2 * dl - 1) as usize] as i32
            })
            .collect()
    }
}
