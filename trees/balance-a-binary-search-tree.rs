/*
  Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.
  A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.

  Example 1:
    Input: root = [1,null,2,null,3,null,4,null,null]
    Output: [2,1,3,null,null,null,4]
    Explanation: This is not the only correct answer, [3,1,4,null,2] is also correct.
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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes = in_order(root, Vec::new());
        create_bst(&nodes, 0, nodes.len())
    }
}

fn in_order(
    node: Option<Rc<RefCell<TreeNode>>>, 
    mut nodes: Vec<Rc<RefCell<TreeNode>>>
) -> Vec<Rc<RefCell<TreeNode>>> {
    if let Some(n) = node {
        nodes = in_order(n.borrow().left.clone(), nodes);
        nodes.push(n.clone());
        nodes = in_order(n.borrow().right.clone(), nodes);
    }
    nodes
}

fn create_bst(
    nodes: &[Rc<RefCell<TreeNode>>],
    start: usize,
    end: usize
) -> Option<Rc<RefCell<TreeNode>>> {
    if start >= end {
        return None;
    }

    let mid = (start + end) / 2;
    let node = Rc::new(RefCell::new(TreeNode {
        val: nodes[mid].borrow().val,
        left: create_bst(nodes, start, mid),
        right: create_bst(nodes, mid + 1, end),
    }));

    Some(node)
}
