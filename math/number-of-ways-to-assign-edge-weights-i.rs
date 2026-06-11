/*
3558. Number of Ways to Assign Edge Weights I

There is an undirected tree with n nodes labeled from 1 to n, rooted at node 1. The tree is represented by a 2D integer array edges of length n - 1, 
where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi.
Initially, all edges have a weight of 0. You must assign each edge a weight of either 1 or 2.
  The cost of a path between any two nodes u and v is the total weight of all edges in the path connecting them.
  Select any one node x at the maximum depth. Return the number of ways to assign edge weights in the path from node 1 to x such that its total cost is odd.
  Since the answer may be large, return it modulo 109 + 7.

Note: Ignore all edges not in the path from node 1 to x.

Example 1:
Input: edges = [[1,2]]
Output: 1
Explanation:
    The path from Node 1 to Node 2 consists of one edge (1 → 2).
    Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.

Example 2:
Input: edges = [[1,2],[1,3],[3,4],[3,5]]
Output: 2
Explanation:
    The maximum depth is 2, with nodes 4 and 5 at the same depth. Either node can be selected for processing.
    For example, the path from Node 1 to Node 4 consists of two edges (1 → 3 and 3 → 4).
    Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.
*/
const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        if n == 1 {
            return 0;
        }

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
            adj[start[u] as usize] = v as u32;
            start[u] += 1;
            adj[start[v] as usize] = u as u32;
            start[v] += 1;
        }

        let mut visited = vec![0u64; (n >> 6) + 1];
        let mut queue = vec![0u32; n];

        queue[0] = 1;
        visited[0] |= 1 << 1;
        
        let (mut head, mut tail) = (0usize, 1usize);
        let mut depth = 0u32;
        
        while head < tail {
            let level_end = tail;
            while head < level_end {
                let u = queue[head] as usize;
                head += 1;
                for &v in &adj[start[u - 1] as usize..start[u] as usize] {
                    let (w, b) = ((v >> 6) as usize, v & 63);
                    if visited[w] >> b & 1 == 0 {
                        visited[w] |= 1 << b;
                        queue[tail] = v;
                        tail += 1;
                    }
                }
            }
            if head < tail {
                depth += 1;
            }
        }

        let mut e = depth - 1;
        let (mut res, mut b) = (1u64, 2u64);

        while e > 0 {
            if e & 1 == 1 {
                res = res * b % MOD;
            }
            b = b * b % MOD;
            e >>= 1;
        }

        res as i32
    }
}
