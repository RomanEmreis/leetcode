/*
  3625. Count Number of Trapezoids II
  
  You are given a 2D integer array points where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
  
  Return the number of unique trapezoids that can be formed by choosing any four distinct points from points.
  
  A trapezoid is a convex quadrilateral with at least one pair of parallel sides. Two lines are parallel if and only if they have the same slope.
  
  Example 1:
  Input: points = [[-3,2],[3,0],[2,3],[3,2],[2,-3]]
  Output: 2
  Explanation:
  There are two distinct ways to pick four points that form a trapezoid:
      The points [-3,2], [2,3], [3,2], [2,-3] form one trapezoid.
      The points [2,3], [3,2], [3,0], [2,-3] form another trapezoid.
  
  Example 2:
  Input: points = [[0,0],[1,0],[0,1],[2,1]]
  Output: 1
  Explanation:
  There is only one trapezoid which can be formed.
*/
public sealed class Solution {
    public int CountTrapezoids(int[][] points) {
        Dictionary<double, List<double>> slopeToIntercept = [];
        Dictionary<double, List<double>> midToSlope = [];

        for (int i = 0; i < points.Length; ++i) {
            int x1 = points[i][0];
            int y1 = points[i][1];
            for (int j = i + 1; j < points.Length; ++j) {
                int x2 = points[j][0];
                int y2 = points[j][1];
                int dx = x1 - x2;
                int dy = y1 - y2;

                double k, b;
                if (x2 == x1) {
                    k = double.PositiveInfinity;
                    b = x1;
                } else {
                    k = (double) (y2 - y1) / (x2 - x1);
                    b = (double) (y1 * dx - x1 * dy) / dx;
                }
                
                double mid = (x1 + x2) * 10000.0 + (y1 + y2);

                if (!slopeToIntercept.ContainsKey(k))
                    slopeToIntercept[k] = [];
                
                if (!midToSlope.ContainsKey(mid))
                    midToSlope[mid] = [];
                
                slopeToIntercept[k].Add(b);
                midToSlope[mid].Add(k);
            }
        }

        int res = 0;

        foreach (var (_, sti) in slopeToIntercept) {
            if (sti.Count == 1)
                continue;
            
            Dictionary<double, int> cnt = [];
            foreach (double bVal in sti)
                cnt[bVal] = cnt.GetValueOrDefault(bVal, 0) + 1;

            int totalSum = 0;
            foreach (var (_, count) in cnt) {
                res += totalSum * count;
                totalSum += count;
            }
        }

        foreach (var (_, mts) in midToSlope) {
            if (mts.Count == 1)
                continue;
            
            Dictionary<double, int> cnt = [];
            foreach (double kVal in mts)
                cnt[kVal] = cnt.GetValueOrDefault(kVal, 0) + 1;
            
            int totalSum = 0;
            foreach (var (_, count) in cnt) {
                res -= totalSum * count;
                totalSum += count;
            }
        }
        
        return res;
    }
}
