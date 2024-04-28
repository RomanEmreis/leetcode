/*
  There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

  You are given the integer n and the array edges where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

  Return an array answer of length n where answer[i] is the sum of the distances between the ith node in the tree and all other nodes.
*/
class Solution {
private:
    unordered_map<int, vector<int>> graph;
    vector<int> result;
    vector<int> count;

    void dfs1(const int& node, const int& parent) {
        for (const int& neighbor : graph[node]) {
            if (neighbor == parent) continue;

            dfs1(neighbor, node);

            count[node] += count[neighbor];
            result[node] += result[neighbor] + count[neighbor];
        }
    }

    void dfs2(const int& node, const int& parent) {
        for (const int& neighbor : graph[node]) {
            if (neighbor == parent) continue;
            
            result[neighbor] = result[node] - count[neighbor] + (count.size() - count[neighbor]);
            dfs2(neighbor, node);
        }
    }

public:
    vector<int> sumOfDistancesInTree(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};

        graph.clear();

        for (const vector<int>& edge : edges) {
            graph[edge[0]].push_back(edge[1]);
            graph[edge[1]].push_back(edge[0]);
        }

        result.resize(n);
        count.resize(n, 1);

        dfs1(0, -1);
        dfs2(0, -1);

        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
