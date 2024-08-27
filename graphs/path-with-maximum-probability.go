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
type Edge struct {
	node     int
	weight   float64
}

type PriorityQueueItem struct {
	node     int
	priority float64
	index    int
}

type PriorityQueue []*PriorityQueueItem

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index, pq[j].index = i, j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*PriorityQueueItem)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func maxProbability(n int, edges [][]int, succProb []float64, start_node int, end_node int) float64 {
    graph := make([][]Edge, n)
	for i, edge := range edges {
		u, v := edge[0], edge[1]
		prob := succProb[i]
		graph[u] = append(graph[u], Edge{v, -math.Log(prob)})
		graph[v] = append(graph[v], Edge{u, -math.Log(prob)})
	}

	dist := make([]float64, n)
	for i := range dist {
		dist[i] = math.Inf(1)
	}
	dist[start_node] = 0

	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	heap.Push(&pq, &PriorityQueueItem{start_node, 0, 0})

	for pq.Len() > 0 {
		item := heap.Pop(&pq).(*PriorityQueueItem)
		node, currDist := item.node, item.priority

		if currDist > dist[node] {
			continue
		}

		for _, edge := range graph[node] {
			neighbor, weight := edge.node, edge.weight
			newDist := dist[node] + weight
			if newDist < dist[neighbor] {
				dist[neighbor] = newDist
				heap.Push(&pq, &PriorityQueueItem{neighbor, newDist, 0})
			}
		}
	}

	if math.IsInf(dist[end_node], 1) {
		return 0.0
	}
	return math.Exp(-dist[end_node])
}
