/*
  3310. Remove Methods From Project
  
  You are maintaining a project that has n methods numbered from 0 to n - 1.
  
  You are given two integers n and k, and a 2D integer array invocations, where invocations[i] = [ai, bi] indicates that method ai invokes method bi.
  There is a known bug in method k. Method k, along with any method invoked by it, either directly or indirectly, are considered suspicious and we aim to remove them.
  
  A group of methods can only be removed if no method outside the group invokes any methods within it.
  
  Return an array containing all the remaining methods after removing all the suspicious methods. You may return the answer in any order. If it is not possible to remove all the suspicious methods, none should be removed.
  
  Example 1:
  Input: n = 4, k = 1, invocations = [[1,2],[0,1],[3,2]]
  Output: [0,1,2,3]
  Explanation:
  Method 2 and method 1 are suspicious, but they are directly invoked by methods 3 and 0, which are not suspicious. We return all elements without removing anything.
  
  Example 2:
  Input: n = 5, k = 0, invocations = [[1,2],[0,2],[0,1],[3,4]]
  Output: [3,4]
  Explanation:
  Methods 0, 1, and 2 are suspicious and they are not directly invoked by any other method. We can remove them.
  
  Example 3:
  Input: n = 3, k = 2, invocations = [[1,2],[0,1],[2,0]]
  Output: []
  Explanation:
  All methods are suspicious. We can remove them.
*/
impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let m = invocations.len();

        let mut offsets = vec![0usize; n + 1];

        for edge in &invocations {
            offsets[edge[0] as usize + 1] += 1;
        }

        for method in 0..n {
            offsets[method + 1] += offsets[method];
        }

        let mut cursor = offsets[..n].to_vec();
        let mut edges = vec![0usize; m];

        for edge in &invocations {
            let caller = edge[0] as usize;
            edges[cursor[caller]] = edge[1] as usize;
            cursor[caller] += 1;
        }

        cursor.clear();
        let mut stack = cursor;

        let mut suspicious = vec![false; n];
        let mut suspicious_count = 1usize;

        suspicious[k] = true;
        stack.push(k);

        while let Some(method) = stack.pop() {
            for &called in &edges[offsets[method]..offsets[method + 1]] {
                if !suspicious[called] {
                    suspicious[called] = true;
                    suspicious_count += 1;
                    stack.push(called);
                }
            }
        }

        for edge in &invocations {
            let caller = edge[0] as usize;
            let called = edge[1] as usize;

            if !suspicious[caller] && suspicious[called] {
                return (0..n as i32).collect();
            }
        }

        let mut res = Vec::with_capacity(n - suspicious_count);

        for method in 0..n {
            if !suspicious[method] {
                res.push(method as i32);
            }
        }

        res
    }
}
