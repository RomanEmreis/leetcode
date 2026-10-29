/*
  3841. Palindromic Path Queries in a Tree
  
  You are given an undirected tree with n nodes labeled 0 to n - 1. This is represented by a 2D array edges of length n - 1, where edges[i] = [ui, vi] indicates an undirected edge between nodes ui and vi.
  You are also given a string s of length n consisting of lowercase English letters, where s[i] represents the character assigned to node i.
  You are also given a string array queries, where each queries[i] is either:
      "update ui c": Change the character at node ui to c. Formally, update s[ui] = c.
      "query ui vi": Determine whether the string formed by the characters on the unique path from ui to vi (inclusive) can be rearranged into a palindrome.
  
  Return a boolean array answer, where answer[j] is true if the jth query of type "query ui vi"​​​​​​​ can be rearranged into a palindrome, and false otherwise.
  
  Example 1:
  Input: n = 3, edges = [[0,1],[1,2]], s = "aac", queries = ["query 0 2","update 1 b","query 0 2"]
  Output: [true,false]
  Explanation:
      "query 0 2": Path 0 → 1 → 2 gives "aac", which can be rearranged to form "aca", a palindrome. Thus, answer[0] = true.
      "update 1 b": Update node 1 to 'b', now s = "abc".
      "query 0 2": Path characters are "abc", which cannot be rearranged to form a palindrome. Thus, answer[1] = false.
  Thus, answer = [true, false].
  
  Example 2:
  Input: n = 4, edges = [[0,1],[0,2],[0,3]], s = "abca", queries = ["query 1 2","update 0 b","query 2 3","update 3 a","query 1 3"]
  Output: [false,false,true]
  Explanation:
      "query 1 2": Path 1 → 0 → 2 gives "bac", which cannot be rearranged to form a palindrome. Thus, answer[0] = false.
      "update 0 b": Update node 0 to 'b', now s = "bbca".
      "query 2 3": Path 2 → 0 → 3 gives "cba", which cannot be rearranged to form a palindrome. Thus, answer[1] = false.
      "update 3 a": Update node 3 to 'a', s = "bbca".
      "query 1 3": Path 1 → 0 → 3 gives "bba", which can be rearranged to form "bab", a palindrome. Thus, answer[2] = true.
  Thus, answer = [false, false, true].
*/
impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut chars = s.into_bytes();
        let mut parent = vec![0usize; n];
        let mut depth = vec![0usize; n];
        let mut tin = vec![0usize; n];
        let mut tout = vec![0usize; n];
        let mut initial_prefix = vec![0u32; n];

        initial_prefix[0] = 1u32 << (chars[0] - b'a');

        let mut stack = vec![(0usize, false)];
        let mut timer = 0usize;

        while let Some((node, exiting)) = stack.pop() {
            if exiting {
                tout[node] = timer - 1;
                continue;
            }

            tin[node] = timer;
            timer += 1;

            stack.push((node, true));

            for &next in graph[node].iter().rev() {
                if next == parent[node] && node != 0 {
                    continue;
                }

                if next == 0 || next == parent[node] {
                    continue;
                }

                parent[next] = node;
                depth[next] = depth[node] + 1;
                initial_prefix[next] = initial_prefix[node] ^ (1u32 << (chars[next] - b'a'));

                stack.push((next, false));
            }
        }

        let mut levels = 1usize;

        while (1usize << levels) <= n {
            levels += 1;
        }

        let mut up = vec![vec![0usize; n]; levels];
        up[0].copy_from_slice(&parent);

        for level in 1..levels {
            for node in 0..n {
                up[level][node] = up[level - 1][up[level - 1][node]];
            }
        }

        fn lca(
            mut u: usize,
            mut v: usize,
            depth: &[usize],
            up: &[Vec<usize>],
        ) -> usize {
            if depth[u] < depth[v] {
                std::mem::swap(&mut u, &mut v);
            }

            let difference = depth[u] - depth[v];

            for level in 0..up.len() {
                if difference & (1usize << level) != 0 {
                    u = up[level][u];
                }
            }

            if u == v {
                return u;
            }

            for level in (0..up.len()).rev() {
                if up[level][u] != up[level][v] {
                    u = up[level][u];
                    v = up[level][v];
                }
            }

            up[0][u]
        }

        let mut fenwick = Fenwick::new(n);
        let mut res = Vec::new();

        for query in queries {
            let mut parts = query.split_ascii_whitespace();
            let operation = parts.next().unwrap();
            let u: usize = parts.next().unwrap().parse().unwrap();

            if operation == "update" {
                let new_char = parts.next().unwrap().as_bytes()[0];
                let delta = (1u32 << (chars[u] - b'a')) ^ (1u32 << (new_char - b'a'));

                if delta != 0 {
                    fenwick.add(tin[u], delta);

                    if tout[u] + 1 < n {
                        fenwick.add(tout[u] + 1, delta);
                    }

                    chars[u] = new_char;
                }
            } else {
                let v: usize = parts.next().unwrap().parse().unwrap();

                let ancestor = lca(u, v, &depth, &up);
                let prefix_u = initial_prefix[u] ^ fenwick.point(tin[u]);
                let prefix_v = initial_prefix[v] ^ fenwick.point(tin[v]);
                let ancestor_mask = 1u32 << (chars[ancestor] - b'a');
                let path_mask = prefix_u ^ prefix_v ^ ancestor_mask;

                res.push(path_mask & path_mask.wrapping_sub(1) == 0);
            }
        }

        res
    }
}

struct Fenwick {
    tree: Vec<u32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, index: usize, value: u32) {
        let mut i = index + 1;

        while i < self.tree.len() {
            self.tree[i] ^= value;
            i += i & i.wrapping_neg();
        }
    }

    fn point(&self, index: usize) -> u32 {
        let mut i = index + 1;
        let mut result = 0;

        while i > 0 {
            result ^= self.tree[i];
            i &= i - 1;
        }

        result
    }
}
