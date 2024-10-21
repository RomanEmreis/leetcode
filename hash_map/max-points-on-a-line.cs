/*
  Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
  
  Example 1:
    Input: points = [[1,1],[2,2],[3,3]]
    Output: 3
  
  Example 2:
    Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
    Output: 4
*/
public class Solution {
    public int MaxPoints(int[][] points) {
        if (points.Length < 2) return points.Length;

        int result = 0;

        Dictionary<(int, int), int> slopes = new();
        for (int i = 0; i < points.Length; ++i) {
            slopes.Clear();
            int duplicate = 1;
            int maxVert = 0;

            for (int j = i + 1; j < points.Length; ++j) {
                int dx = points[j][0] - points[i][0];
                int dy = points[j][1] - points[i][1];

                if (dx == 0 && dy == 0) {
                    ++duplicate;
                    continue;
                }

                if (dx == 0) {
                    ++maxVert;
                    continue;
                }

                int gcd = Gcd(dx, dy);
                dx /= gcd;
                dy /= gcd;

                if (dx < 0) {
                    dx = -dx;
                    dy = -dy;
                }

                var slope = (dx, dy);
                if (!slopes.ContainsKey(slope)) slopes[slope] = 1;
                else ++slopes[slope];
            }

            int max = maxVert;
            foreach (var (_, v) in slopes) {
                if (v > max) {
                    max = v;
                }
            }
            result = Math.Max(result, max + duplicate);
        }

        return result;
    }

    int Gcd(int x, int y) {
        while (y != 0) {
            int t = y;
            y = x % y;
            x = t;
        }
        return Math.Abs(x);
    }
}
