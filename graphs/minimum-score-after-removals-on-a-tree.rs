/*
  2322. Minimum Score After Removals on a Tree
  
  There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.
  You are given a 0-indexed integer array nums of length n where nums[i] represents the value of the ith node. You are also given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
  
  Remove two distinct edges of the tree to form three connected components. For a pair of removed edges, the following steps are defined:
  
  Get the XOR of all the values of the nodes for each of the three components respectively.
    The difference between the largest XOR value and the smallest XOR value is the score of the pair.
    For example, say the three components have the node values: [4,5,7], [1,9], and [3,3,3]. The three XOR values are 4 ^ 5 ^ 7 = 6, 1 ^ 9 = 8, and 3 ^ 3 ^ 3 = 3. The largest XOR value is 8 and the smallest XOR value is 3. The score is then 8 - 3 = 5.
  
  Return the minimum score of any possible pair of edge removals on the given tree.
  
  Example 1:
  Input: nums = [1,5,5,4,11], edges = [[0,1],[1,2],[1,3],[3,4]]
  Output: 9
  Explanation: The diagram above shows a way to make a pair of removals.
  - The 1st component has nodes [1,3,4] with values [5,4,11]. Its XOR value is 5 ^ 4 ^ 11 = 10.
  - The 2nd component has node [0] with value [1]. Its XOR value is 1 = 1.
  - The 3rd component has node [2] with value [5]. Its XOR value is 5 = 5.
  The score is the difference between the largest and smallest XOR value which is 10 - 1 = 9.
  It can be shown that no other pair of removals will obtain a smaller score than 9.
  
  Example 2:
  Input: nums = [5,5,2,4,4,2], edges = [[0,1],[1,2],[5,2],[4,3],[1,3]]
  Output: 0
  Explanation: The diagram above shows a way to make a pair of removals.
  - The 1st component has nodes [3,4] with values [4,4]. Its XOR value is 4 ^ 4 = 0.
  - The 2nd component has nodes [1,0] with values [5,5]. Its XOR value is 5 ^ 5 = 0.
  - The 3rd component has nodes [2,5] with values [2,2]. Its XOR value is 2 ^ 2 = 0.
  The score is the difference between the largest and smallest XOR value which is 0 - 0 = 0.
  We cannot obtain a smaller score than 0.
*/
impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut sum = vec![0; n];
        let mut tin = vec![0; n];
        let mut tout = vec![0; n];
        let mut adj = vec![Vec::with_capacity(n); n];
        let mut cnt = 0;

        for e in edges {
            let u = e[0];
            let v = e[1];
            adj[u as usize].push(v as usize);
            adj[v as usize].push(u as usize);
        }

        fn dfs(
            x: usize,
            fa: usize,
            cnt: &mut i32,
            nums: &Vec<i32>,
            sum: &mut Vec<i32>,
            tin: &mut Vec<i32>,
            tout: &mut Vec<i32>,
            adj: &Vec<Vec<usize>>,
        ) {
            tin[x] = *cnt;
            *cnt += 1;
            sum[x] = nums[x];
            for &y in &adj[x] {
                if y == fa {
                    continue;
                }
                dfs(y, x, cnt, nums, sum, tin, tout, adj);
                sum[x] ^= sum[y];
            }
            tout[x] = *cnt;
        }

        fn calc(a: i32, b: i32, c: i32) -> i32 {
            a.max(b.max(c)) - a.min(b.min(c))
        }

        dfs(0, usize::MAX, &mut cnt, &nums, &mut sum, &mut tin, &mut tout, &adj);

        let mut res = i32::MAX;
        for u in 1..n {
            for v in (u + 1)..n {
                if tin[v] > tin[u] && tin[v] < tout[u] {
                    let a = sum[0] ^ sum[u];
                    let b = sum[u] ^ sum[v];
                    let c = sum[v];
                    res = res.min(calc(a, b, c));
                } else if tin[u] > tin[v] && tin[u] < tout[v] {
                    let a = sum[0] ^ sum[v];
                    let b = sum[v] ^ sum[u];
                    let c = sum[u];
                    res = res.min(calc(a, b, c));
                } else {
                    let a = sum[0] ^ sum[u] ^ sum[v];
                    let b = sum[u];
                    let c = sum[v];
                    res = res.min(calc(a, b, c));
                }
            }
        }

        res
    }
}
