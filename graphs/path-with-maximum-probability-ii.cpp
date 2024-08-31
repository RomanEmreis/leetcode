/*
  You are given an undirected weighted graph of n nodes (0-indexed), represented by an edge list where edges[i] = [a, b] is an undirected edge connecting the nodes a and b with a probability of success of traversing that edge succProb[i].
  Given two nodes start and end, find the path with the maximum probability of success to go from start to end and return its success probability.
  
  If there is no path from start to end, return 0. Your answer will be accepted if it differs from the correct answer by at most 1e-5. 
  
  Example 1:  
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
    Output: 0.25000
    Explanation: There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
  
  Example 2:
    Input: n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
    Output: 0.30000
  
  Example 3:
    Input: n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
    Output: 0.00000
    Explanation: There is no path between 0 and 2.
*/
static const int _ = []{cin.tie(0);ios::sync_with_stdio(0);return 0;}();
class Solution {
 public:
  double maxProbability(int n, vector<vector<int>>& edges, vector<double>& succProb, int start, int end) {
    // {a: [(b, probability_ab)]}
    vector<vector<pair<int, double>>> graph(n);
    // (the probability to reach u, u)
    priority_queue<pair<double, int>> maxHeap;
    maxHeap.emplace(1.0, start);
    vector<bool> seen(n);

    for (int i = 0; i < edges.size(); ++i) {
      const int u = edges[i][0];
      const int v = edges[i][1];
      const double prob = succProb[i];
      graph[u].emplace_back(v, prob);
      graph[v].emplace_back(u, prob);
    }

    while (!maxHeap.empty()) {
      const auto [prob, u] = maxHeap.top();
      maxHeap.pop();
      if (u == end)
        return prob;
      if (seen[u])
        continue;
      seen[u] = true;
      for (const auto& [nextNode, edgeProb] : graph[u]) {
        if (seen[nextNode])
          continue;
        maxHeap.emplace(prob * edgeProb, nextNode);
      }
    }

    return 0;
  }
};
