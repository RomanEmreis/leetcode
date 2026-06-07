/*
  2196. Create Binary Tree From Descriptions
  
  You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,
  
      If isLefti == 1, then childi is the left child of parenti.
      If isLefti == 0, then childi is the right child of parenti.
  
  Construct the binary tree described by descriptions and return its root.
  
  The test cases will be generated such that the binary tree is valid.
  
  Example 1:
  Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
  Output: [50,20,80,15,17,19]
  Explanation: The root node is the node with value 50 since it has no parent.
  The resulting binary tree is shown in the diagram.
  
  Example 2:
  Input: descriptions = [[1,2,1],[2,3,0],[3,4,1]]
  Output: [1,2,null,null,3,4]
  Explanation: The root node is the node with value 1 since it has no parent.
  The resulting binary tree is shown in the diagram.
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
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map = HashMap::with_capacity(descriptions.len() * 2);
        let mut children = HashSet::with_capacity(descriptions.len());

        for desc in descriptions {
            let parent_value = desc[0];
            let child_value = desc[1];
            let is_left = desc[2] == 1;

            let child_node = node_map
                .entry(child_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_value))))
                .clone();
            
            let parent_node = node_map
                .entry(parent_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_value))));

            if is_left {
                parent_node.borrow_mut().left = Some(child_node);
            } else {
                parent_node.borrow_mut().right = Some(child_node);
            }

            children.insert(child_value);
        }

        node_map
            .iter()
            .find(|(value, _) | !children.contains(value))
            .map(|(_, node)| node.clone())
    }
}
