/*
  3337. Total Characters in String After Transformations II

  You are given a string s consisting of lowercase English letters, an integer t representing the number of transformations to perform, and an array nums of size 26. 
  In one transformation, every character in s is replaced according to the following rules:
  
  Replace s[i] with the next nums[s[i] - 'a'] consecutive characters in the alphabet. For example, if s[i] = 'a' and nums[0] = 3, 
  the character 'a' transforms into the next 3 consecutive characters ahead of it, which results in "bcd".
  The transformation wraps around the alphabet if it exceeds 'z'. For example, if s[i] = 'y' and nums[24] = 3, the character 'y' 
  transforms into the next 3 consecutive characters ahead of it, which results in "zab".
  Return the length of the resulting string after exactly t transformations.
  
  Since the answer may be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: s = "abcyy", t = 2, nums = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]
  Output: 7
  Explanation:
  First Transformation (t = 1):
  'a' becomes 'b' as nums[0] == 1
  'b' becomes 'c' as nums[1] == 1
  'c' becomes 'd' as nums[2] == 1
  'y' becomes 'z' as nums[24] == 1
  'y' becomes 'z' as nums[24] == 1
  String after the first transformation: "bcdzz"
  Second Transformation (t = 2):
  'b' becomes 'c' as nums[1] == 1
  'c' becomes 'd' as nums[2] == 1
  'd' becomes 'e' as nums[3] == 1
  'z' becomes 'ab' as nums[25] == 2
  'z' becomes 'ab' as nums[25] == 2
  String after the second transformation: "cdeabab"
  Final Length of the string: The string is "cdeabab", which has 7 characters.
  
  Example 2:
  Input: s = "azbk", t = 1, nums = [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]
  Output: 8
  Explanation:
  First Transformation (t = 1):
  'a' becomes 'bc' as nums[0] == 2
  'z' becomes 'ab' as nums[25] == 2
  'b' becomes 'cd' as nums[1] == 2
  'k' becomes 'lm' as nums[10] == 2
  String after the first transformation: "bcabcdlm"
  Final Length of the string: The string is "bcabcdlm", which has 8 characters.
*/
public class Solution {
    private const int MOD = 1_000_000_007;
    private const int M = 26;

    public int LengthAfterTransformations(string s, int t, IList<int> nums) {
        int[] cnt = new int[M];
        foreach (char c in s) {
            cnt[c - 'a']++;
        }

        int[][] matrix = new int[M][];
        for (int i = 0; i < M; ++i) {
            matrix[i] = new int[M];
            for (int j = 1; j <= nums[i]; ++j) {
                matrix[i][(i + j) % M] = 1;
            }
        }

        int[][] cntMat = new int[1][];
        cntMat[0] = new int[M];
        Array.Copy(cnt, cntMat[0], M);

        int[][] factor = MatPow(matrix, t);
        int[][] result = MatMul(cntMat, factor);

        int ans = 0;
        foreach (int x in result[0]) {
            ans = (ans + x) % MOD;
        }

        return ans;
    }

    private static int[][] MatMul(int[][] a, int[][] b) {
        int n = a.Length;
        int p = b.Length;
        int q = b[0].Length;

        int[][] res = new int[n][];
        for (int i = 0; i < n; ++i) {
            res[i] = new int[q];
        }

        for (int i = 0; i < n; ++i) {
            for (int k = 0; k < p; ++k) {
                if (a[i][k] != 0) {
                    for (int j = 0; j < q; ++j) {
                        long mul = (long)a[i][k] * b[k][j] % MOD;
                        res[i][j] = (int)((res[i][j] + mul) % MOD);
                    }
                }
            }
        }

        return res;
    }

    private static int[][] MatPow(int[][] mat, int power) {
        int[][] res = new int[M][];
        for (int i = 0; i < M; ++i) {
            res[i] = new int[M];
            res[i][i] = 1;
        }

        while (power > 0) {
            if ((power & 1) == 1)
                res = MatMul(res, mat);
            mat = MatMul(mat, mat);
            power >>= 1;
        }

        return res;
    }
}
