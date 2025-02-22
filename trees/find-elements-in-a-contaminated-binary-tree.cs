/*
  1261. Find Elements in a Contaminated Binary Tree
  
  Given a binary tree with the following rules:
      root.val == 0
      For any treeNode:
          If treeNode.val has a value x and treeNode.left != null, then treeNode.left.val == 2 * x + 1
          If treeNode.val has a value x and treeNode.right != null, then treeNode.right.val == 2 * x + 2
  
  Now the binary tree is contaminated, which means all treeNode.val have been changed to -1.
  Implement the FindElements class:
      FindElements(TreeNode* root) Initializes the object with a contaminated binary tree and recovers it.
      bool find(int target) Returns true if the target value exists in the recovered binary tree.
  
  Example 1:
  Input
  ["FindElements","find","find"]
  [[[-1,null,-1]],[1],[2]]
  Output
  [null,false,true]
  Explanation
  FindElements findElements = new FindElements([-1,null,-1]); 
  findElements.find(1); // return False 
  findElements.find(2); // return True 
  
  Example 2:
  Input
  ["FindElements","find","find","find"]
  [[[-1,-1,-1,-1,-1]],[1],[3],[5]]
  Output
  [null,true,true,false]
  Explanation
  FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
  findElements.find(1); // return True
  findElements.find(3); // return True
  findElements.find(5); // return False
  
  Example 3:
  Input
  ["FindElements","find","find","find","find"]
  [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
  Output
  [null,true,false,false,true]
  Explanation
  FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
  findElements.find(2); // return True
  findElements.find(3); // return False
  findElements.find(4); // return False
  findElements.find(5); // return True
*/
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     public int val;
 *     public TreeNode left;
 *     public TreeNode right;
 *     public TreeNode(int val=0, TreeNode left=null, TreeNode right=null) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
public sealed class FindElements {
    private readonly HashSet<int> _values;
    public FindElements(TreeNode root) {
        _values = [];
        Recover(root, 0);
    }
    
    public bool Find(int target) {
        return _values.Contains(target);
    }

    private void Recover(TreeNode root, int val) {
        if (root is null) return;
        root.val = val;
        _values.Add(val);
        Recover(root.left, 2 * val + 1);
        Recover(root.right, 2 * val + 2);
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * FindElements obj = new FindElements(root);
 * bool param_1 = obj.Find(target);
 */
