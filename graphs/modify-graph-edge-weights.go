/*
  You are given an undirected weighted connected graph containing n nodes labeled from 0 to n - 1, and an integer array edges where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.
  Some edges have a weight of -1 (wi = -1), while others have a positive weight (wi > 0).
  Your task is to modify all edges with a weight of -1 by assigning them positive integer values in the range [1, 2 * 109] so that the shortest distance between the nodes source and destination becomes equal to an integer target.
  If there are multiple modifications that make the shortest distance between source and destination equal to target, any of them will be considered correct.
  
  Return an array containing all edges (even unmodified ones) in any order if it is possible to make the shortest distance from source to destination equal to target, or an empty array if it's impossible.
  
  Note: You are not allowed to modify the weights of edges with initial positive weights.
  
  Example 1:
    Input: n = 5, edges = [[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]], source = 0, destination = 1, target = 5
    Output: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
    Explanation: The graph above shows a possible modification to the edges, making the distance from 0 to 1 equal to 5.

  Example 2:
    Input: n = 3, edges = [[0,1,-1],[0,2,5]], source = 0, destination = 2, target = 6
    Output: []
    Explanation: The graph above contains the initial edges. It is not possible to make the distance from 0 to 2 equal to 6 by modifying the edge with weight -1. So, an empty array is returned.
  
  Example 3:
    Input: n = 4, edges = [[1,0,4],[1,2,3],[2,3,5],[0,3,-1]], source = 0, destination = 2, target = 6
    Output: [[1,0,4],[1,2,3],[2,3,5],[0,3,1]]
    Explanation: The graph above shows a modified graph having the shortest distance from 0 to 2 as 6.
*/
type Pair struct {
	Node, Dist int
}

type PriorityQueue []Pair

func (pq PriorityQueue) Len() int { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].Dist < pq[j].Dist
}
func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}
func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(Pair))
}
func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	x := old[n-1]
	*pq = old[0 : n-1]
	return x
}

func dijkstra(graph [][]Pair, src, dst int) int {
	dist := make([]int, len(graph))
	for i := range dist {
		dist[i] = math.MaxInt32
	}
	dist[src] = 0

	pq := make(PriorityQueue, 0)
	heap.Push(&pq, Pair{src, 0})

	for pq.Len() > 0 {
		p := heap.Pop(&pq).(Pair)
		u := p.Node
		if p.Dist > dist[u] {
			continue
		}

		for _, info := range graph[u] {
			v, weight := info.Node, info.Dist
			if dist[u]+weight < dist[v] {
				dist[v] = dist[u] + weight
				heap.Push(&pq, Pair{v, dist[v]})
			}
		}
	}

	return dist[dst]
}

func modifiedGraphEdges(n int, edges [][]int, source, destination, target int) [][]int {
	const maxWeight = 2000000000
	graph := make([][]Pair, n)
	for _, edge := range edges {
		u, v, w := edge[0], edge[1], edge[2]
		if w == -1 {
			continue
		}
		graph[u] = append(graph[u], Pair{v, w})
		graph[v] = append(graph[v], Pair{u, w})
	}

	if distToDestination := dijkstra(graph, source, destination); distToDestination < target {
		return [][]int{}
	} else if distToDestination == target {
		for i := range edges {
			if edges[i][2] == -1 {
				edges[i][2] = maxWeight
			}
		}
		return edges
	}

	for i, edge := range edges {
		u, v, w := edge[0], edge[1], &edges[i][2]
		if *w != -1 {
			continue
		}
		*w = 1
		graph[u] = append(graph[u], Pair{v, 1})
		graph[v] = append(graph[v], Pair{u, 1})
		if distToDestination := dijkstra(graph, source, destination); distToDestination <= target {
			*w += target - distToDestination
			for j := i + 1; j < len(edges); j++ {
				if edges[j][2] == -1 {
					edges[j][2] = maxWeight
				}
			}
			return edges
		}
	}
	return [][]int{}
}
