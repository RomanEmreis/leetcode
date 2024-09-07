/*
  Given a binary tree root and a linked list with head as the first node. 
  Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
  
  In this context downward path means a path that starts at some node and goes downwards.

  Example 1:
    Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
    Output: true
    Explanation: Nodes in blue form a subpath in the binary Tree. 
*/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if  root.is_none() {
            return false;
        }

        let root_ref = root.as_ref().unwrap().borrow();

        Self::dfs(&head, &root) || 
        Self::is_sub_path(head.clone(), root_ref.left.clone()) || 
        Self::is_sub_path(head.clone(), root_ref.right.clone())
    }

    pub fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }

        let root_ref = root.as_ref().unwrap().borrow();
        let head_ref = head.as_ref().unwrap();

        root_ref.val == head_ref.val && (
            Self::dfs(&head_ref.next, &root_ref.left) || 
            Self::dfs(&head_ref.next, &root_ref.right)
        )
    }
}
