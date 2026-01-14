/*
  3454. Separate Squares II
  
  You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
  Find the minimum y-coordinate value of a horizontal line such that the total area covered by squares above the line equals the total area covered by squares below the line.
  Answers within 10-5 of the actual answer will be accepted.
  
  Note: Squares may overlap. Overlapping areas should be counted only once in this version.
  
  Example 1:
  Input: squares = [[0,0,1],[2,2,1]]
  Output: 1.00000
  Explanation:
  Any horizontal line between y = 1 and y = 2 results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
  
  Example 2:
  Input: squares = [[0,0,2],[1,1,1]]
  Output: 1.00000
  Explanation:
  Since the blue square overlaps with the red square, it will not be counted again. Thus, the line y = 1 splits the squares into two equal parts.
*/
public class Solution {
    public double SeparateSquares(int[][] squares) {
        List<(int, int, int, int)> events = [];
        SortedSet<int> xs = [];

        foreach (int[] sq in squares) {
            int x = sq[0];
            int y = sq[1];
            int l = sq[2];

            events.Add((y, 1, x, x + l));
            events.Add((y + l, -1, x, x + l));

            xs.Add(x);
            xs.Add(x + l);
        }

        events.Sort((a, b) => a.Item1.CompareTo(b.Item1));

        int[] xss = xs.ToArray();

        double halfArea = GetArea(events, xss) / 2.0;
        long area = 0;
        int prevY = 0;

        SegmentTree tree = new(xss);

        foreach (var (y, delta, xl, xr) in events) {
            long areaGrain = tree.CoveredWidth * (long)(y - prevY);
            if (area + areaGrain >= halfArea)
                return prevY + (halfArea - area) / tree.CoveredWidth;
            
            area += areaGrain;
            tree.Add(xl, xr, delta);
            prevY = y;
        }

        return -1.0;
    }

    private static double GetArea(List<(int, int, int, int)> events, int[] xs) {
        long totalArea = 0;
        int prevY = 0;

        SegmentTree tree = new(xs);

        foreach (var (y, delta, xl, xr) in events) {
            totalArea += tree.CoveredWidth * (long)(y - prevY);
            tree.Add(xl, xr, delta);
            prevY = y;
        }

        return totalArea;
    }
}

public sealed class SegmentTree {
    private readonly int n;
    private readonly int[] xs;
    private readonly int[] coveredCount;
    private readonly int[] coveredWidth;

    public SegmentTree(int[] xs) {
        this.xs = xs;
        n = xs.Length - 1;
        coveredCount = new int[4 * n];
        coveredWidth = new int[4 * n];
    }

    public long CoveredWidth => 1L * coveredWidth[0];

    public void Add(int i, int j, int val) => Add(0, 0, n - 1, i, j, val);

    private void Add(int idx, int lo, int hi, int i, int j, int val) {
        if (j <= xs[lo] || xs[hi + 1] <= i)
            return;
        if (i <= xs[lo] && xs[hi + 1] <= j) {
            coveredCount[idx] += val;
        } else {
            int m = (lo + hi) / 2;
            Add(2 * idx + 1, lo, m, i, j, val);
            Add(2 * idx + 2, m + 1, hi, i, j, val);
        }
            
        if (coveredCount[idx] > 0) {
          coveredWidth[idx] = xs[hi + 1] - xs[lo];
        } else if (lo == hi) {
          coveredWidth[idx] = 0;
        } else {
          coveredWidth[idx] = coveredWidth[2 * idx + 1] + coveredWidth[2 * idx + 2];
        }
    }
}
