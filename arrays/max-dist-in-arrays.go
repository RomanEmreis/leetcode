/*
  You are given m arrays, where each array is sorted in ascending order.
  You can pick up two integers from two different arrays (each array picks one) and calculate the distance. We define the distance between two integers a and b to be their absolute difference |a - b|.
  
  Return the maximum distance.

  Example 1:
    Input: arrays = [[1,2,3],[4,5],[1,2,3]]
    Output: 4
    Explanation: One way to reach the maximum distance 4 is to pick 1 in the first or third array and pick 5 in the second array.

  Example 2:
    Input: arrays = [[1],[1]]
    Output: 0
*/
func maxDistance(arrays [][]int) int {
    maxDistance := 0
	minElement := arrays[0][0]
	maxElement := arrays[0][len(arrays[0])-1]

	for i := 1; i < len(arrays); i++ {
		currentMin := arrays[i][0]
		currentMax := arrays[i][len(arrays[i])-1]

		maxDistance = max(maxDistance, max(abs(currentMax-minElement), abs(maxElement-currentMin)))

		minElement = min(minElement, currentMin)
		maxElement = max(maxElement, currentMax)
	}

	return maxDistance
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func min(x, y int) int {
	if x < y {
		return x
	}
	return y
}
