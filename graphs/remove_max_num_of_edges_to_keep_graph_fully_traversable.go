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
type UnionFind struct {
    parent, size []int;
    count int;
}

func createUnionFind(n int) *UnionFind {
	parent := make([]int, n + 1);
	size := make([]int, n + 1);

	for i := 1; i <= n; i++ {
		parent[i] = i
		size[i] = 1
	}

	return &UnionFind{parent, size, n};
}

func (s *UnionFind) find(x int) int {
    if x != s.parent[x] {
        s.parent[x] = s.find(s.parent[x]);
    }

    return s.parent[x];
}

func (s *UnionFind) union(x, y int) bool {
    rootX, rootY := s.find(x), s.find(y);
    if rootX == rootY {
        return false;
    }

    if s.size[rootX] > s.size[rootY] {
        s.parent[rootY] = rootX;
        s.size[rootX] += rootY;
    } else {
        s.parent[rootX] = rootY;
        s.size[rootY] += rootX;
    }

    s.count--;

    return true;
}

func maxNumEdgesToRemove(n int, edges [][]int) int {
    result := 0;
    s := createUnionFind(n);

    for _, edge := range edges {
        if edge[0] == 3 && !s.union(edge[1], edge[2]) {
            result++;
        }
    }

    parent2 := append([]int(nil), s.parent...);
    size2 := append([]int(nil), s.size...);
    count2 := s.count;

    for _, edge := range edges {
        if edge[0] == 1 && !s.union(edge[1], edge[2]) {
            result++;
        }
    }

    if s.count != 1 {
        return -1;
    }

    s.parent = parent2;
    s.size = size2;
    s.count = count2;

    for _, edge := range edges {
        if edge[0] == 2 && !s.union(edge[1], edge[2]) {
            result++;
        }
    }

    if s.count != 1 {
        return -1;
    }

    return result;
}
