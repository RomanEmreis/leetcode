/*
  3650. Minimum Cost Path with Edge Reversals
  
  You are given a directed, weighted graph with n nodes labeled from 0 to n - 1, and an array edges 
  where edges[i] = [ui, vi, wi] represents a directed edge from node ui to node vi with cost wi.
  Each node ui has a switch that can be used at most once: when you arrive at ui and have not yet used its switch, 
  you may activate it on one of its incoming edges vi → ui reverse that edge to ui → vi and immediately traverse it.
  The reversal is only valid for that single move, and using a reversed edge costs 2 * wi.
  
  Return the minimum total cost to travel from node 0 to node n - 1. If it is not possible, return -1.
  
  Example 1:
  Input: n = 4, edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]]
  Output: 5
  Explanation:
      Use the path 0 → 1 (cost 3).
      At node 1 reverse the original edge 3 → 1 into 1 → 3 and traverse it at cost 2 * 1 = 2.
      Total cost is 3 + 2 = 5.
  
  Example 2:
  Input: n = 4, edges = [[0,2,1],[2,1,1],[1,3,1],[2,3,3]]
  Output: 3
  Explanation:
      No reversal is needed. Take the path 0 → 2 (cost 1), then 2 → 1 (cost 1), then 1 → 3 (cost 1).
      Total cost is 1 + 1 + 1 = 3.
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];

        edges.into_iter().for_each(|e| {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];

            g[u].push((v, w));
            g[v].push((u, 2 * w));
        });

        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));

        while let Some(Reverse((d, node))) = heap.pop() {
            if d != dist[node] {
                continue;
            }
            if node == n - 1 {
                return d;
            }

            for (to, w) in g[node].iter() {
                let nd = d + w;
                if nd < dist[*to] {
                    dist[*to] = nd;
                    heap.push(Reverse((nd, *to)));
                }
            }
        }

        -1
    }
}
