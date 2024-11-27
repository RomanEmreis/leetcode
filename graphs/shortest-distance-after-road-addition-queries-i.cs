/*
  3243. Shortest Distance After Road Addition Queries I
  
  You are given an integer n and a 2D integer array queries.
  There are n cities numbered from 0 to n - 1. Initially, there is a unidirectional road from city i to city i + 1 for all 0 <= i < n - 1.
  queries[i] = [ui, vi] represents the addition of a new unidirectional road from city ui to city vi. After each query, you need to find the length of the shortest path from city 0 to city n - 1.
  Return an array answer where for each i in the range [0, queries.length - 1], answer[i] is the length of the shortest path from city 0 to city n - 1 after processing the first i + 1 queries.
  
  Example 1:
  Input: n = 5, queries = [[2,4],[0,2],[0,4]]
  Output: [3,2,1]
  Explanation:
  After the addition of the road from 2 to 4, the length of the shortest path from 0 to 4 is 3.
  After the addition of the road from 0 to 2, the length of the shortest path from 0 to 4 is 2.
  After the addition of the road from 0 to 4, the length of the shortest path from 0 to 4 is 1.
*/
public class Solution {
    public int[] ShortestDistanceAfterQueries(int n, int[][] queries) {
        int destination = n - 1;
        int[] result = new int[queries.Length];

        List<int>[] graph = new List<int>[n];
        for (int i = 0; i < n; ++i) {
            graph[i] = [i + 1];
        }

        for (int i = 0; i < queries.Length; ++i) {
            graph[queries[i][0]].Add(queries[i][1]);
            result[i] = Bfs(0);
        }

        return result;

        int Bfs(int i) {
            Queue<int> q = [];
            q.Enqueue(i);

            Span<bool> visited = stackalloc bool[n];
            visited[i] = true;

            for (int d = 0;; ++d) {
                for (int k = q.Count; k > 0; --k) {
                    int u = q.Dequeue();
                    if (u == destination) return d;

                    foreach (int v in graph[u]) {
                        if (!visited[v]) {
                            q.Enqueue(v);
                            visited[v] = true;
                        }
                    }
                }
            }
        }
    }
}
