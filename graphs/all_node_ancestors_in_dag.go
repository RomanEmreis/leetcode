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
func getAncestors(n int, edges [][]int) [][]int {
    graph := make([][]int, n);

    for i := 0; i < n; i++ {
        graph[i] = make([]int, 0);
    }
    for _, edge := range edges {
        graph[edge[1]] = append(graph[edge[1]], edge[0]);
    }

	ancestors := make([]map[int]struct{}, n);
	for i := 0; i < n; i++ {
		ancestors[i] = make(map[int]struct{});
	}

	visited := make([]bool, n);
	for i := 0; i < n; i++ {
		if !visited[i] {
			dfs(i, graph, ancestors, visited);
		}
	}

	result := make([][]int, n);
	for i := 0; i < n; i++ {
		result[i] = make([]int, 0, len(ancestors[i]));
		for ancestor := range ancestors[i] {
			result[i] = append(result[i], ancestor);
		}

		sort.Ints(result[i]);
	}

	return result;
}

func dfs(node int, graph [][]int, ancestors []map[int]struct{}, visited []bool) {
	visited[node] = true;
	for _, parent := range graph[node] {
		if !visited[parent] {
			dfs(parent, graph, ancestors, visited);
		}
		ancestors[node][parent] = struct{}{};
		for ancestor := range ancestors[parent] {
			ancestors[node][ancestor] = struct{}{};
		}
	}
}
