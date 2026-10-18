/*
  1569. Number of Ways to Reorder Array to Get Same BST
  
  Given an array nums that represents a permutation of integers from 1 to n. We are going to construct a binary search tree (BST) by inserting the elements of nums in order into an initially empty BST. 
  Find the number of different ways to reorder nums so that the constructed BST is identical to that formed from the original array nums.
      For example, given nums = [2,1,3], we will have 2 as the root, 1 as a left child, and 3 as a right child. The array [2,3,1] also yields the same BST but [3,2,1] yields a different BST.
  
  Return the number of ways to reorder nums such that the BST formed is identical to the original BST formed from nums.
  
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: nums = [2,1,3]
  Output: 1
  Explanation: We can reorder nums to be [2,3,1] which will yield the same BST. 
  There are no other ways to reorder nums which will yield the same BST.
  
  Example 2:
  Input: nums = [3,4,5,1,2]
  Output: 5
  Explanation: The following 5 arrays will yield the same BST: 
  [3,1,2,4,5]
  [3,1,4,2,5]
  [3,1,4,5,2]
  [3,4,1,2,5]
  [3,4,1,5,2]
  
  Example 3:
  Input: nums = [1,2,3]
  Output: 0
  Explanation: There are no other orderings of nums that will yield the same BST.
*/
impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn mod_pow(mut base: i64, mut exponent: i64) -> i64 {
        let mut res = 1i64;

        while exponent > 0 {
            if exponent & 1 == 1 {
                res = res * base % Self::MOD;
            }

            base = base * base % Self::MOD;
            exponent >>= 1;
        }

        res
    }

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut position = vec![0usize; n];

        for (index, &value) in nums.iter().enumerate() {
            position[value as usize - 1] = index;
        }

        let mut left = vec![n; n];
        let mut right = vec![n; n];
        let mut stack = Vec::with_capacity(n);

        for node in 0..n {
            let mut last_removed = n;

            while let Some(&previous) = stack.last() {
                if position[previous] < position[node] {
                    break;
                }

                last_removed = stack.pop().unwrap();
            }

            if let Some(&parent) = stack.last() {
                right[parent] = node;
            }

            left[node] = last_removed;
            stack.push(node);
        }

        let root = stack[0];

        let mut factorial = vec![1i64; n + 1];
        let mut inverse_factorial = vec![1i64; n + 1];

        for value in 1..=n {
            factorial[value] = factorial[value - 1] * value as i64 % Self::MOD;
        }

        inverse_factorial[n] = Self::mod_pow(factorial[n], Self::MOD - 2);

        for value in (1..=n).rev() {
            inverse_factorial[value - 1] = inverse_factorial[value] * value as i64 % Self::MOD;
        }

        let mut order = Vec::with_capacity(n);
        let mut pending = Vec::with_capacity(n);
        pending.push(root);

        while let Some(node) = pending.pop() {
            order.push(node);

            if left[node] != n {
                pending.push(left[node]);
            }

            if right[node] != n {
                pending.push(right[node]);
            }
        }

        let mut subtree_size = vec![0usize; n];
        let mut ways = vec![1i64; n];

        for &node in order.iter().rev() {
            let left_size = if left[node] == n {
                0
            } else {
                subtree_size[left[node]]
            };

            let right_size = if right[node] == n {
                0
            } else {
                subtree_size[right[node]]
            };

            let left_ways = if left[node] == n {
                1
            } else {
                ways[left[node]]
            };

            let right_ways = if right[node] == n {
                1
            } else {
                ways[right[node]]
            };

            let interleavings =
                factorial[left_size + right_size]
                    * inverse_factorial[left_size]
                    % Self::MOD
                    * inverse_factorial[right_size]
                    % Self::MOD;

            ways[node] = interleavings * left_ways % Self::MOD * right_ways % Self::MOD;
            subtree_size[node] = left_size + right_size + 1;
        }

        ((ways[root] - 1 + Self::MOD) % Self::MOD) as i32
    }
}
