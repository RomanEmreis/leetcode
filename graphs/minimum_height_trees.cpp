class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};

        unordered_map<int, vector<int>> graph;
        vector<int> degree(n, 0);

        for (const vector<int>& edge : edges) {
            degree[edge[0]]++;
            degree[edge[1]]++;

            graph[edge[0]].push_back(edge[1]);
            graph[edge[1]].push_back(edge[0]);
        }

        queue<int> leaves;
        for (int i = 0; i < degree.size(); ++i) {
            if (degree[i] == 1) leaves.push(i);
        }

        int remaining = n;
        while (remaining > 2) {
            int leavesCount = leaves.size();
            remaining -= leavesCount;

            for (int i = 0; i < leavesCount; ++i) {
                int leaf = leaves.front();
                leaves.pop();

                for (const int& neighbor : graph[leaf]) {
                    if (--degree[neighbor] == 1) leaves.push(neighbor);
                }
            }
        }

        vector<int> result;
        for ( ;!leaves.empty(); leaves.pop()) {
            result.push_back(leaves.front());
        }

        return result;
    }
};
