/*
  You are given an integer n denoting the number of cities in a country. The cities are numbered from 0 to n - 1.
  You are also given a 2D integer array roads where roads[i] = [ai, bi] denotes that there exists a bidirectional road connecting cities ai and bi.

  You need to assign each city with an integer value from 1 to n, where each value can only be used once. The importance of a road is then defined as the sum of the values of the two cities it connects.
  Return the maximum total importance of all roads possible after assigning the values optimally.

  Example:
    Input: n = 5, roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
    Output: 43
    Explanation: The figure above shows the country and the assigned values of [2,4,5,3,1].
    - The road (0,1) has an importance of 2 + 4 = 6.
    - The road (1,2) has an importance of 4 + 5 = 9.
    - The road (2,3) has an importance of 5 + 3 = 8.
    - The road (0,2) has an importance of 2 + 5 = 7.
    - The road (1,3) has an importance of 4 + 3 = 7.
    - The road (2,4) has an importance of 5 + 1 = 6.
  The total importance of all roads is 6 + 9 + 8 + 7 + 7 + 6 = 43.
  It can be shown that we cannot obtain a greater total importance than 43.
*/
public class Solution {
    public long MaximumImportance(int n, int[][] roads) {
        long result = 0;

        int[] weights = new int[n];
        foreach (int[] road in roads) {
            ++weights[road[0]];
            ++weights[road[1]];
        }

        Span<int> cities = stackalloc int[n];
        for (int i = 0; i < n; ++i) {
            cities[i] = i;
        }

        cities.Sort(Comparer<int>.Create(Compare));

        for (int i = 0; i < n; ++i) {
            result += (long) (n - i) * weights[cities[i]];
        }

        return result;

        int Compare(int x, int y) => weights[y].CompareTo(weights[x]);
    }
}
