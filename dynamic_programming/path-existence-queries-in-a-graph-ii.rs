/*
3534. Path Existence Queries in a Graph II

You are given an integer n representing the number of nodes in a graph, labeled from 0 to n - 1.
You are also given an integer array nums of length n and an integer maxDiff.

An undirected edge exists between nodes i and j if the absolute difference between nums[i] and nums[j] is at most maxDiff (i.e., |nums[i] - nums[j]| <= maxDiff).
You are also given a 2D integer array queries. For each queries[i] = [ui, vi], find the minimum distance between nodes ui and vi. If no path exists between the two nodes, return -1 for that query.

Return an array answer, where answer[i] is the result of the ith query.

Note: The edges between the nodes are unweighted.

Example 1:
Input: n = 5, nums = [1,8,3,4,2], maxDiff = 3, queries = [[0,3],[2,4]]
Output: [1,1]
Explanation:
The resulting graph is:
Query	Shortest Path	Minimum Distance
[0, 3]	0 → 3	1
[2, 4]	2 → 4	1

Thus, the output is [1, 1].

Example 2:
Input: n = 5, nums = [5,3,1,9,10], maxDiff = 2, queries = [[0,1],[0,2],[2,3],[4,3]]
Output: [1,2,-1,1]
Explanation:
The resulting graph is:
Query	Shortest Path	Minimum Distance
[0, 1]	0 → 1	1
[0, 2]	0 → 1 → 2	2
[2, 3]	None	-1
[4, 3]	3 → 4	1

Thus, the output is [1, 2, -1, 1].

Example 3:
Input: n = 3, nums = [3,6,1], maxDiff = 1, queries = [[0,0],[0,1],[1,2]]
Output: [0,-1,-1]
Explanation:
There are no edges between any two nodes because:
    Nodes 0 and 1: |nums[0] - nums[1]| = |3 - 6| = 3 > 1
    Nodes 0 and 2: |nums[0] - nums[2]| = |3 - 1| = 2 > 1
    Nodes 1 and 2: |nums[1] - nums[2]| = |6 - 1| = 5 > 1

Thus, no node can reach any other node, and the output is [0, -1, -1].
*/
impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let max_diff = max_diff as i64;

        let mut order: Vec<u32> = (0..n as u32).collect();
        order.sort_unstable_by_key(|&i| nums[i as usize]);

        let mut pos = vec![0u32; n];
        for (sorted_i, &orig_i) in order.iter().enumerate() {
            pos[orig_i as usize] = sorted_i as u32;
        }

        let val: Vec<i32> = order.iter().map(|&i| nums[i as usize]).collect();
        drop(order);

        let mut l = 0usize;
        while (1usize << l) < n {
            l += 1;
        }

        let log = l + 1;
        let mut st = vec![0u32; log * n];
        let mut r = 0usize;
        for i in 0..n {
            if r < i {
                r = i;
            }
            while r + 1 < n && (val[r + 1] as i64 - val[i] as i64) <= max_diff {
                r += 1;
            }
            st[i] = r as u32;
        }

        for j in 1..log {
            let (prev, cur) = st.split_at_mut(j * n);
            let prev_row = &prev[(j - 1) * n..j * n];
            let cur_row = &mut cur[..n];
            for i in 0..n {
                cur_row[i] = prev_row[prev_row[i] as usize];
            }
        }

        queries
            .iter()
            .map(|q| {
                let (mut a, mut b) = (pos[q[0] as usize], pos[q[1] as usize]);
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                if a == b {
                    return 0;
                }
                let mut cur = a as usize;
                let mut steps: i32 = 0;
                for j in (0..log).rev() {
                    let nxt = st[j * n + cur];
                    if nxt < b {
                        cur = nxt as usize;
                        steps += 1 << j;
                    }
                }
                if st[cur] >= b {
                    steps + 1
                } else {
                    -1
                }
            })
            .collect()
    }
}
