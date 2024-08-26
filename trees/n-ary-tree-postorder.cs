/*
// Definition for a Node.
public class Node {
    public int val;
    public IList<Node> children;

    public Node() {}

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, IList<Node> _children) {
        val = _val;
        children = _children;
    }
}
*/

public class Solution {
    public IList<int> Postorder(Node root) {
        List<int> result = [];
        Traverse(root, result);
        return result;
    }
    
    private static void Traverse(Node root, List<int> result) {
        if (root == null) return;

        foreach (Node child in root.children) {
            Traverse(child, result);
        }
        result.Add(root.val);
    }
}
