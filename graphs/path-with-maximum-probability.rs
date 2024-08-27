/*
  You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
  
  Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
  
  If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5.

  Example 1:
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
    Output: 0.25000
    Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
  
  Example 2:
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
    Output: 0.30000
*/
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::f64::{self, consts};

struct State {
    cost: f64,
    position: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.position == other.position
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Creating a min-heap (smaller costs have higher priority)
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let start = start_node as usize;
        let end = end_node as usize;

        let mut graph = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            let weight = -succ_prob[i].ln();
            graph[u].push((v, weight));
            graph[v].push((u, weight));
        }

        let mut min_heap = BinaryHeap::new();
        let mut costs = vec![f64::INFINITY; n];
        costs[start] = 0.0;
        min_heap.push(State { cost: 0.0, position: start });

        while let Some(State { cost, position }) = min_heap.pop() {
            if position == end {
                return (-costs[position]).exp();  // Use the final cost to compute probability
            }

            if cost > costs[position] {
                continue;
            }

            for &(next, weight) in &graph[position] {
                let next_cost = cost + weight;
                if next_cost < costs[next] {
                    costs[next] = next_cost;
                    min_heap.push(State { cost: next_cost, position: next });
                }
            }
        }

        0.0  // Return 0.0 if no path is found
    }
}
