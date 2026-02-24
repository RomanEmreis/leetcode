/*
  1022. Sum of Root To Leaf Binary Numbers
  
  You are given the root of a binary tree where each node has a value 0 or 1. Each root-to-leaf path represents a binary number starting with the most significant bit.
      For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.
  
  For all leaves in the tree, consider the numbers represented by the path from the root to that leaf. Return the sum of these numbers.
  
  The test cases are generated so that the answer fits in a 32-bits integer.
  
  Example 1:
  Input: root = [1,0,1,0,1,0,1]
  Output: 22
  Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
  
  Example 2:
  Input: root = [0]
  Output: 0
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
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        dfs(root, 0, &mut res);       
        res
    }
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut val: i32, res: &mut i32) {
    let n = match &node {
        Some(n) => n.borrow(),
        None => return
    };

    val = val * 2 + n.val;

    if n.left.is_none() && n.right.is_none() {
        *res += val;
    }

    dfs(n.left.clone(), val, res);
    dfs(n.right.clone(), val, res);
}
