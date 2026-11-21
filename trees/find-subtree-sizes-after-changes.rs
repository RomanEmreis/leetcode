/*
  3331. Find Subtree Sizes After Changes
  
  You are given a tree rooted at node 0 that consists of n nodes numbered from 0 to n - 1. 
  The tree is represented by an array parent of size n, where parent[i] is the parent of node i. Since node 0 is the root, parent[0] == -1.
  
  You are also given a string s of length n, where s[i] is the character assigned to node i.
  
  We make the following changes on the tree one time simultaneously for all nodes x from 1 to n - 1:
      Find the closest node y to node x such that y is an ancestor of x, and s[x] == s[y].
      If node y does not exist, do nothing.
      Otherwise, remove the edge between x and its current parent and make node y the new parent of x by adding an edge between them.
  
  Return an array answer of size n where answer[i] is the size of the subtree rooted at node i in the final tree.
  
  Example 1:
  Input: parent = [-1,0,0,1,1,1], s = "abaabc"
  Output: [6,3,1,1,1,1]
  Explanation:
  The parent of node 3 will change from node 1 to node 0.
  
  Example 2:
  Input: parent = [-1,0,4,0,1], s = "abbba"
  Output: [5,2,1,1,1]
  Explanation:
  The following changes will happen at the same time:
      The parent of node 4 will change from node 1 to node 0.
      The parent of node 2 will change from node 4 to node 1.
*/
impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        const ENTER: i32 = -2;

        let n = parent.len();
        let letters = s.as_bytes();

        let mut head = vec![-1i32; n];
        let mut next = vec![-1i32; n];

        for v in 1..n {
            let p = parent[v] as usize;
            next[v] = head[p];
            head[p] = v as i32;
        }

        let mut last = [-1i32; 26];
        let mut res = vec![1i32; n];

        let mut stack = Vec::with_capacity(n);
        stack.push((0i32, ENTER));

        while let Some((node, previous)) = stack.pop() {
            let u = node as usize;
            let c = (letters[u] - b'a') as usize;

            if previous == ENTER {
                let saved = last[c];
                last[c] = node;

                stack.push((node, saved));

                let mut child = head[u];
                while child != -1 {
                    stack.push((child, ENTER));
                    child = next[child as usize];
                }
            } else {
                last[c] = previous;

                let target = if previous != -1 {
                    previous
                } else {
                    parent[u]
                };

                if target != -1 {
                    let size = res[u];
                    res[target as usize] += size;
                }
            }
        }

        res
    }
}
