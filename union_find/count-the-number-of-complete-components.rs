/*
  2685. Count the Number of Complete Components
  
  You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1. 
  You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting vertices ai and bi.
  
  Return the number of complete connected components of the graph.
  
  A connected component is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
  A connected component is said to be complete if there exists an edge between every pair of its vertices. 
  
  Example 1:
  Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
  Output: 3
  Explanation: From the picture above, one can see that all of the components of this graph are complete.
  
  Example 2:
  Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
  Output: 1
  Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between every pair of two vertices. 
  On the other hand, the component containing vertices 3, 4, and 5 is not complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.
*/
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![1; n],
            components: n,
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }

        self.components -= 1;
        true
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);

        edges.iter().for_each(|e| {
            let u = e[0] as usize;
            let v = e[1] as usize;
            uf.union(u, v);
        });

        let mut node_count = vec![0usize; n];

        for i in 0..n {
            let root = uf.find(i);
            node_count[root] += 1;
        }

        let mut edge_count = vec![0usize; n];

        edges.into_iter().for_each(|e| {
            let u = e[0] as usize;
            let root = uf.find(u);
            edge_count[root] += 1;
        });

        let mut res = 0;

        for root in 0..n {
            let nodes = node_count[root];
            if nodes == 0 {
                continue;
            }

            let expected_edges = nodes * (nodes - 1) / 2;
            if edge_count[root] == expected_edges {
                res += 1;
            }
        }

        res
    }
}
