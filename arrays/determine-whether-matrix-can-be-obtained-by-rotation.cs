/*
  1886. Determine Whether Matrix Can Be Obtained By Rotation
  
  Given two n x n binary matrices mat and target, return true if it is possible to make mat equal to
  target by rotating mat in 90-degree increments, or false otherwise.
  
  Example 1:
  Input: mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
  Output: true
  Explanation: We can rotate mat 90 degrees clockwise to make mat equal target.
  
  Example 2:
  Input: mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
  Output: false
  Explanation: It is impossible to make mat equal to target by rotating mat.
  
  Example 3:
  Input: mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
  Output: true
  Explanation: We can rotate mat 90 degrees clockwise two times to make mat equal target.
*/
public class Solution {
    public bool FindRotation(int[][] mat, int[][] target) {
        int n = mat.Length;
        for (int i = 0; i < 4; ++i) {
            if (MatrixEqual(mat, target))
                return true;
            Rotate(mat, n);
        }
        return false;
    }

    private static bool MatrixEqual(int[][] a, int[][] b) {
        for (int i = 0; i < a.Length; i++)
            if (!a[i].AsSpan().SequenceEqual(b[i].AsSpan())) 
                return false;
        return true;
    }

    private static void Rotate(int[][] mat, int n) {
        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n; ++j) {
                mat[i][j] ^= mat[j][i];
                mat[j][i] ^= mat[i][j];
                mat[i][j] ^= mat[j][i];
            }
        }

        for (int i = 0; i < n; ++i) {
            Array.Reverse(mat[i]);
        }
    }
}
