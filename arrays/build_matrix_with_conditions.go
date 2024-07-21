/*
  You are given a positive integer k. You are also given:
    a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
    a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].
  The two arrays contain integers from 1 to k.

  You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.

  The matrix should also satisfy the following conditions:
    The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
    The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.

  Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.

  Example 1:
    Input: k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
    Output: [[3,0,0],[0,0,1],[0,2,0]]
    Explanation: The diagram above shows a valid example of a matrix that satisfies all the conditions.
    The row conditions are the following:
    - Number 1 is in row 1, and number 2 is in row 2, so 1 is above 2 in the matrix.
    - Number 3 is in row 0, and number 2 is in row 2, so 3 is above 2 in the matrix.
    The column conditions are the following:
    - Number 2 is in column 1, and number 1 is in column 2, so 2 is left of 1 in the matrix.
    - Number 3 is in column 0, and number 2 is in column 1, so 3 is left of 2 in the matrix.
    Note that there may be multiple correct answers.

  Example 2:
    Input: k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
    Output: []
    Explanation: From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
    No matrix can satisfy all the conditions, so we return the empty matrix.
*/
func buildMatrix(k int, rowConditions [][]int, colConditions [][]int) [][]int {
    rowGraph := make([][]int, k + 1);
    colGraph := make([][]int, k + 1);

    for _, condition := range rowConditions {
        rowGraph[condition[0]] = append(rowGraph[condition[0]], condition[1]);
    }

    for _, condition := range colConditions {
        colGraph[condition[0]] = append(colGraph[condition[0]], condition[1]);
    }

    rowSorted := topologicalSort(&rowGraph, &k);
    colSorted := topologicalSort(&colGraph, &k);

    if len(rowSorted) == 0 || len(colSorted) == 0 {
        return [][]int{};
    }

    result := make([][]int, k);
    rowMap := make(map[int]int);
    colMap := make(map[int]int);

    for i := 0; i < k; i++ {
        result[i] = make([]int, k);
        rowMap[rowSorted[i]] = i;
        colMap[colSorted[i]] = i;
    }

    for i := 1; i <= k; i++ {
        result[rowMap[i]][colMap[i]] = i;
    }

    return result;
}

func topologicalSort(graph *[][]int, k *int) []int {
    degree := make([]int, *k + 1);
    sorted := []int{};
    q := []int{};

    for i := 1; i <= *k; i++ {
        for _, j := range (*graph)[i] {
            degree[j]++;
        }
    }
    for i := 1; i <= *k; i++ {
        if degree[i] == 0 {
            q = append(q, i);
        }
    }

    for len(q) > 0 {
        node := q[0];
        q = q[1:];
        sorted = append(sorted, node);
        for _, n := range (*graph)[node] { 
            degree[n]--;
            if degree[n] == 0 {
                q = append(q, n);
            }
        }
    }

    if len(sorted) == *k {
        return sorted;
    }

    return []int{};
}
