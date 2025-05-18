/*
  1931. Painting a Grid With Three Different Colors
  
  You are given two integers m and n. Consider an m x n grid where each cell is initially white. You can paint each cell red, green, or blue. All cells must be painted.
  Return the number of ways to color the grid with no two adjacent cells having the same color. Since the answer can be very large, return it modulo 109 + 7.
  
  Example 1:
  Input: m = 1, n = 1
  Output: 3
  Explanation: The three possible colorings are shown in the image above.
  
  Example 2:
  Input: m = 1, n = 2
  Output: 6
  Explanation: The six possible colorings are shown in the image above.
  
  Example 3:
  Input: m = 5, n = 5
  Output: 580986
*/
public class Solution {
    public int ColorTheGrid(int m, int n) {
        const int mod = (int) 1e9 + 7;
        int mx = (int) Math.Pow(3, m);
        HashSet<int> valid = [];

        Span<int> dp = stackalloc int[mx];
        for (int i = 0; i < mx; ++i) {
            if (F1(i)) {
                valid.Add(i);
                dp[i] = 1;
            }
        }

        Dictionary<int, List<int>> d = [];
        foreach (int i in valid) {
            foreach (int j in valid) {
                if (F2(i, j)) {
                    if (d.ContainsKey(i)) d[i].Add(j);
                    else d[i] = [j];
                }
            }
        }

        for (int k = 1; k < n; ++k) {
            Span<int> g = stackalloc int[mx];
            foreach (int i in valid) {
                foreach (int j in d[i]) {
                    g[i] = (g[i] + dp[j]) % mod;
                }
            }
            dp = g;
        }

        long result = 0;
        foreach (int x in dp) {
            result = (result + x) % mod;
        }
        return (int)(result % mod);

        bool F1(int x) {
            int last = -1;
            for (int i = 0; i < m; ++i) {
                if (x % 3 == last) return false;
                last = x % 3;
                x /= 3;
            }
            return true;
        }

        bool F2(int x, int y) {
            for (int i = 0; i < m; ++i) {
                if (x % 3 == y % 3) return false;
                x /= 3;
                y /= 3;
            }
            return true;
        }
    }
}
