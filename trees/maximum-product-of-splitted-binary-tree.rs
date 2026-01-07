/*
  1339. Maximum Product of Splitted Binary Tree
  
  Given the root of a binary tree, split the binary tree into two subtrees by removing one edge such that the product of the sums of the subtrees is maximized.
  Return the maximum product of the sums of the two subtrees. Since the answer may be too large, return it modulo 109 + 7.
  
  Note that you need to maximize the answer before taking the mod and not after taking it.
  
  Example 1:
  Input: root = [1,2,3,4,5,6]
  Output: 110
  Explanation: Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
  
  Example 2:
  Input: root = [1,null,2,3,4,null,null,5,6]
  Output: 90
  Explanation: Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
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

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = Vec::new();
        let mut res = 0;    
        let total = tree_sum(&root, &mut sums) as i64;
        for sum in sums.into_iter() {
            let sum = sum as i64;
            res = res.max(sum * (total - sum));
        }
        (res % MOD) as i32
    }
}

fn tree_sum(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sums: &mut Vec<i32>) -> i32 {
    let node = match &node {
        Some(n) => n.borrow(),
        None => return 0
    };

    let left = tree_sum(&node.left, sums);
    let right = tree_sum(&node.right, sums);
    let sum = left + right + node.val;
    sums.push(sum);
    sum
}
