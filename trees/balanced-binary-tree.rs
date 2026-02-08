/*
  110. Balanced Binary Tree
  
  Given a binary tree, determine if it is .
  
  Example 1:
  Input: root = [3,9,20,null,null,15,7]
  Output: true
  
  Example 2:
  Input: root = [1,2,2,3,3,null,null,4,4]
  Output: false
  
  Example 3:
  Input: root = []
  Output: true
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::max_depth(root) != -1
    }

    fn max_depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = match &node {
            Some(node) => node.borrow(),
            None => return 0
        };

        let l = Self::max_depth(node.left.clone());
        let r = Self::max_depth(node.right.clone());

        if l == -1 || r == -1 || (l - r).abs() > 1 {
            return -1;
        }

        1 + l.max(r)
    }
}
