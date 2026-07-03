/*
  3620. Network Recovery Pathways
  
  You are given a directed acyclic graph of n nodes numbered from 0 to n − 1. This is represented by a 2D array edges of length m, 
  where edges[i] = [ui, vi, costi] indicates a one‑way communication from node ui to node vi with a recovery cost of costi.
  Some nodes may be offline. You are given a boolean array online where online[i] = true means node i is online. Nodes 0 and n − 1 are always online.
  
  A path from 0 to n − 1 is valid if:
      All intermediate nodes on the path are online.
      The total recovery cost of all edges on the path does not exceed k.
  
  For each valid path, define its score as the minimum edge‑cost along that path.
  
  Return the maximum path score (i.e., the largest minimum-edge cost) among all valid paths. If no valid path exists, return -1.
  
  Example 1:
  Input: edges = [[0,1,5],[1,3,10],[0,2,3],[2,3,4]], online = [true,true,true,true], k = 10
  Output: 3
  Explanation:
      The graph has two possible routes from node 0 to node 3:
          Path 0 → 1 → 3
              Total cost = 5 + 10 = 15, which exceeds k (15 > 10), so this path is invalid.
          Path 0 → 2 → 3
              Total cost = 3 + 4 = 7 <= k, so this path is valid.
              The minimum edge‐cost along this path is min(3, 4) = 3.
      There are no other valid paths. Hence, the maximum among all valid path‐scores is 3.
  
  Example 2:
  Input: edges = [[0,1,7],[1,4,5],[0,2,6],[2,3,6],[3,4,2],[2,4,6]], online = [true,true,true,false,true], k = 12
  Output: 6
  Explanation:
      Node 3 is offline, so any path passing through 3 is invalid.
      Consider the remaining routes from 0 to 4:
          Path 0 → 1 → 4
              Total cost = 7 + 5 = 12 <= k, so this path is valid.
              The minimum edge‐cost along this path is min(7, 5) = 5.
          Path 0 → 2 → 3 → 4
              Node 3 is offline, so this path is invalid regardless of cost.
          Path 0 → 2 → 4
              Total cost = 6 + 6 = 12 <= k, so this path is valid.
              The minimum edge‐cost along this path is min(6, 6) = 6.
      Among the two valid paths, their scores are 5 and 6. Therefore, the answer is 6.
*/
impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();

        let mut raw = Vec::with_capacity(edges.len());
        let mut costs = Vec::new();
        let mut outdeg = vec![0usize; n];
        let mut indeg = vec![0usize; n];

        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];

            if online[u] && online[v] && (w as i64) <= k {
                raw.push((u, v, w));
                costs.push(w);
                outdeg[u] += 1;
                indeg[v] += 1;
            }
        }

        if raw.is_empty() {
            return -1;
        }

        costs.sort_unstable();
        costs.dedup();

        let mut offsets = vec![0usize; n + 1];
        for i in 0..n {
            offsets[i + 1] = offsets[i] + outdeg[i];
        }

        let mut cursor = offsets.clone();
        let mut to = vec![0usize; raw.len()];
        let mut weight = vec![0i32; raw.len()];

        for (u, v, w) in raw {
            let idx = cursor[u];
            cursor[u] += 1;
            to[idx] = v;
            weight[idx] = w;
        }

        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            if indeg[i] == 0 {
                q.push_back(i);
            }
        }

        let mut topo = Vec::with_capacity(n);
        while let Some(u) = q.pop_front() {
            topo.push(u);

            for e in offsets[u]..offsets[u + 1] {
                let v = to[e];
                indeg[v] -= 1;
                if indeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        const INF: i64 = i64::MAX / 4;
        let mut dist = vec![INF; n];

        let mut check = |min_edge: i32| -> bool {
            dist.fill(INF);
            dist[0] = 0;

            for &u in &topo {
                let du = dist[u];

                if u == n - 1 {
                    return du <= k;
                }

                if du > k {
                    continue;
                }

                for e in offsets[u]..offsets[u + 1] {
                    if weight[e] >= min_edge {
                        let v = to[e];
                        let nd = du + weight[e] as i64;

                        if nd < dist[v] && nd <= k {
                            dist[v] = nd;
                        }
                    }
                }
            }

            false
        };

        let mut lo = 0usize;
        let mut hi = costs.len();
        let mut ans = -1;

        while lo < hi {
            let mid = (lo + hi) / 2;

            if check(costs[mid]) {
                ans = costs[mid];
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        ans
    }
}
