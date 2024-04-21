class Solution {
public:
    bool validPath(int n, vector<vector<int>>& edges, int source, int destination) {
        if (edges.size() == 0) return true;

        unordered_map<int, vector<int>> graph;
        for (int i = 0; i < edges.size(); ++i) {
            graph[edges[i][0]].push_back(edges[i][1]);
            graph[edges[i][1]].push_back(edges[i][0]);
        }

        unordered_set<int> visited;
        queue<int> q;
        q.push(source);

        while (!q.empty()) {
            int current = q.front();
            q.pop();
            if (current == destination) return true;

            for (int neighbor : graph[current]) {
                if (!visited.contains(neighbor)) {
                    visited.insert(neighbor);
                    q.push(neighbor);
                }
            }
        }

        return false;
    }
};
