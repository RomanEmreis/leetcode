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
public class Solution {
    public IList<IList<int>> CombinationSum2(int[] candidates, int target) {
        Array.Sort(candidates);
        List<IList<int>> result = [];
        List<int> comb = [];

        Backtrack(candidates, target, 0, comb, result);

        return result;
    }

    private static void Backtrack(int[] candidates, int target, int start, List<int> comb, List<IList<int>> result) {
        if (target == 0) {
            result.Add([..comb]);
            return;
        }

        for (int i = start; i < candidates.Length; ++i) {
            if (i > start && candidates[i] == candidates[i - 1]) continue;
            if (candidates[i] > target) break;

            comb.Add(candidates[i]);
            Backtrack(candidates, target - candidates[i], i + 1, comb, result);
            comb.RemoveAt(comb.Count - 1);
        }
    }
}
