/*
  You are given a positive integer n representing the number of nodes of a Directed Acyclic Graph (DAG). The nodes are numbered from 0 to n - 1 (inclusive).
  You are also given a 2D integer array edges, where edges[i] = [fromi, toi] denotes that there is a unidirectional edge from fromi to toi in the graph.

  Return a list answer, where answer[i] is the list of ancestors of the ith node, sorted in ascending order.

  A node u is an ancestor of another node v if u can reach v via a set of edges.

  Example:
    Input: n = 8, edgeList = [[0,3],[0,4],[1,3],[2,4],[2,7],[3,5],[3,6],[3,7],[4,6]]
    Output: [[],[],[],[0,1],[0,2],[0,1,3],[0,1,2,3,4],[0,1,2,3]]
  Explanation:
    The above diagram represents the input graph.
    - Nodes 0, 1, and 2 do not have any ancestors.
    - Node 3 has two ancestors 0 and 1.
    - Node 4 has two ancestors 0 and 2.
    - Node 5 has three ancestors 0, 1, and 3.
    - Node 6 has five ancestors 0, 1, 2, 3, and 4.
    - Node 7 has four ancestors 0, 1, 2, and 3.
*/
class Solution {
private:
    void dfs(const int& node, const vector<vector<int>>& graph, vector<vector<int>>& result, vector<bool>& visited) {
        visited[node] = true;
        set<int> s;
        for (const int& parent : graph[node]) {
            if (!visited[parent]) dfs(parent, graph, result, visited);

            s.insert(parent);
            for (const int& p : result[parent]) s.insert(p);
        }

        vector<int> ancestors(s.begin(), s.end());
        result[node] = ancestors;
    }

public:
    vector<vector<int>> getAncestors(int n, vector<vector<int>>& edges) {
        vector<vector<int>> result(n);
        vector<vector<int>> graph(n);

        for (vector<int>& edge : edges) graph[edge[1]].push_back(edge[0]);

        vector<bool> visited(n, false);
        for (int i = 0; i < n; ++i) {
            if (!visited[i]) dfs(i, graph, result, visited);
        }

        return result;
    }
};

int init = [] {
	ios_base::sync_with_stdio(false);
	cin.tie(nullptr);
    cout.tie(nullptr);
	return 0;
}();
