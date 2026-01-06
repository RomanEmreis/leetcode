/*
  1161. Maximum Level Sum of a Binary Tree
  
  Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
  
  Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
  
  Example 1:
  Input: root = [1,7,0,7,-8,null,null]
  Output: 2
  Explanation: 
  Level 1 sum = 1.
  Level 2 sum = 7 + 0 = 7.
  Level 3 sum = 7 + -8 = -1.
  So we return the level with the maximum sum which is level 2.
  
  Example 2:
  Input: root = [989,null,10250,98693,-89388,null,null,null,-32127]
  Output: 2
*/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = Vec::new();
        let mut q = VecDeque::new();

        q.push_back((0, root.clone()));

        while let Some((level, node)) = q.pop_front() {
            let node = match &node {
                Some(node) => node.borrow(),
                None => continue
            };

            if sum.len() == level {
                sum.push(node.val);
            } else {
                sum[level] += node.val;
            }

            q.push_back((level + 1, node.left.clone()));
            q.push_back((level + 1, node.right.clone()));
        }

        let mut res = 0;
        let mut max = i32::MIN;

        for (i, s) in sum.into_iter().enumerate() {
            if (max < s) {
                max = s;
                res = i;
            }
        }

        res as i32 + 1
    }
}
