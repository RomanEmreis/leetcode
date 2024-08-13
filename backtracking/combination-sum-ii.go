/*
  Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
  Each number in candidates may only be used once in the combination.

  Note: The solution set must not contain duplicate combinations.

  Example 1:
    Input: candidates = [10,1,2,7,6,1,5], target = 8
    Output: 
    [
    [1,1,6],
    [1,2,5],
    [1,7],
    [2,6]
    ]
*/
func combinationSum2(candidates []int, target int) [][]int {
    sort.Ints(candidates);
    result := make([][]int, 0);
    combination := make([]int, 0);

    backtrack(candidates, target, 0, &combination, &result);

    return result;
}

func backtrack(candidates []int, target int, startIndex int, combination *[]int, results *[][]int) {
	if target == 0 {
		comboCopy := make([]int, len(*combination));
		copy(comboCopy, *combination);
		*results = append(*results, comboCopy);
		return;
	}

	for i := startIndex; i < len(candidates); i++ {
		if i > startIndex && candidates[i] == candidates[i-1] {
			continue;
		}
		if candidates[i] > target {
			break;
		}
		*combination = append(*combination, candidates[i]);
		backtrack(candidates, target-candidates[i], i+1, combination, results);
		*combination = (*combination)[:len(*combination)-1];
	}
}
