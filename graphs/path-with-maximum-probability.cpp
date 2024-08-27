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
*/
struct State {
    double cost;
    int position;

    bool operator>(const State& other) const {
        return cost > other.cost;
    }
};

class Solution {
public:
    double maxProbability(int n, vector<vector<int>>& edges, vector<double>& succProb, int start_node, int end_node) {
        vector<vector<pair<int, double>>> graph(n);

        for (size_t i = 0; i < edges.size(); ++i) {
            int u = edges[i][0];
            int v = edges[i][1];
            double prob = succProb[i];
            double logProb = -log(prob);
            graph[u].emplace_back(v, logProb);
            graph[v].emplace_back(u, logProb);
        }

        vector<double> costs(n, INFINITY);
        costs[start_node] = 0.0;

        priority_queue<State, vector<State>, greater<>> pq;
        pq.push({0.0, start_node});

        while(!pq.empty()) {
            State curr = pq.top();
            pq.pop();

            int position = curr.position;
            double cost = curr.cost;

            if (position == end_node) return exp(-costs[position]);

            for (auto& next : graph[position]) {
                double new_cost = cost + next.second;
                if (new_cost < costs[next.first]) {
                    costs[next.first] = new_cost;
                    pq.push({new_cost, next.first});
                }
            }
        }

        return 0.0;
    }
};
