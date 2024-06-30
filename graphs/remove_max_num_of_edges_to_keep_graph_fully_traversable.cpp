/*
  Alice and Bob have an undirected graph of n nodes and three types of edges:
    Type 1: Can be traversed by Alice only. 
    Type 2: Can be traversed by Bob only.
    Type 3: Can be traversed by both Alice and Bob.
  Given an array edges where edges[i] = [typei, ui, vi] represents a bidirectional edge of type typei between nodes ui and vi, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob.
  The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.
  Return the maximum number of edges you can remove, or return -1 if Alice and Bob cannot fully traverse the graph. 

  Example:
    Input: n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
    Output: 2
    Explanation: If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
*/
class Solution {
private:
    vector<int> parent;
    vector<long long> size;
    int count;

    int find(int& x) {
        if (x != parent[x]) {
            parent[x] = find(parent[x]);
        }

        return parent[x];
    }

    bool unionSet(int& x, int& y) {
        int rootX = find(x);
        int rootY = find(y);

        if (rootX == rootY) return false;

        if (size[rootX] > size[rootY]) {
            parent[rootY] = rootX;
            size[rootX] += rootY;
        } else {
            parent[rootX] = rootY;
            size[rootY] += rootX;
        }

        --count;

        return true;
    }

public:
    int maxNumEdgesToRemove(int n, vector<vector<int>>& edges) {
        int result = 0;
        parent.resize(n + 1);
        size.resize(n + 1);
        count = n;

        for (int i = 1; i <= n; ++i) {
            parent[i] = i;
            size[i] = 1;
        }

        for (vector<int>& edge : edges) {
            if (edge[0] == 3 && !unionSet(edge[1], edge[2])) ++result;
        }

        vector<int> parent2 = parent;
        vector<long long> size2 = size;
        int count2 = count;

        for (vector<int>& edge : edges) {
            if (edge[0] == 1 && !unionSet(edge[1], edge[2])) ++result;
        }

        if (count != 1) return -1;

        parent = parent2;
        size = size2;
        count = count2;

        for (vector<int>& edge : edges) {
            if (edge[0] == 2 && !unionSet(edge[1], edge[2])) ++result;
        }

        if (count != 1) return -1;

        return result;
    }
};

int init = [] {
	ios_base::sync_with_stdio(false);
	cin.tie(nullptr);
    cout.tie(nullptr);
	return 0;
}();
