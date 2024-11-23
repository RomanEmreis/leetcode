/*
  You are given an m x n matrix of characters box representing a side-view of a box. Each cell of the box is one of the following:
    A stone '#'
    A stationary obstacle '*'
    Empty '.'
  The box is rotated 90 degrees clockwise, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity does not affect the obstacles' positions, and the inertia from the box's rotation does not affect the stones' horizontal positions.
  It is guaranteed that each stone in box rests on an obstacle, another stone, or the bottom of the box.
  
  Return an n x m matrix representing the box after the rotation described above.
  
  Example 1:
  Input: box = [["#",".","#"]]
  Output: [["."],
           ["#"],
           ["#"]]
*/
public class Solution {
    public char[][] RotateTheBox(char[][] box) {
        int m = box.Length;
        int n = box[0].Length;

        char[][] result = new char[n][];
        for (int i = 0; i < n; ++i) {
            result[i] = new char[m];
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                result[j][m - i - 1] = box[i][j];
            } 
        }

        for (int j = 0; j < m; ++j) {
            Queue<int> q = [];
            for (int i = n - 1; i >= 0; --i) {
                if (result[i][j] == '*') q.Clear();
                else if (result[i][j] == '.') q.Enqueue(i);
                else if (q.Count > 0) {
                    result[q.Dequeue()][j] = '#';
                    result[i][j] = '.';
                    q.Enqueue(i);
                }
            }
        }

        return result;
    }
}
