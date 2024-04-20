public class Solution {
    public int[][] FindFarmland(int[][] land) {
        if (land.Length == 0 || land[0].Length == 0) return Array.Empty<int[]>();

        List<int[]> result = [];

        for (int i = 0; i < land.Length; ++i) {
            for (int j = 0; j < land[i].Length; ++j) {
                if (land[i][j] == 1) {
                    result.Add(CalculateArea(land, i, j));
                }
            }
        }

        return result.ToArray();
    }

    private static int[] CalculateArea(int[][] land, int x, int y) {
        int r = x, c = y;

        while (r < land.Length - 1 && land[r + 1][y] == 1) ++r;
        while (c < land[x].Length - 1 && land[x][c + 1] == 1) ++c;

        for (int i = x; i <= r; ++i) {
            for (int j = y; j <= c; ++j) {
                land[i][j] = 0;
            }
        }

        return [x, y, r, c];
    }
}
