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
class Solution {
private:
    static void backtrack(vector<int>& candidates, int target, int start, vector<int>& comb, vector<vector<int>>& result) {
        if (target == 0) {
            result.push_back(comb);
            return;
        }

        for (int i = start; i < candidates.size(); ++i) {
            if (i > start && candidates[i] == candidates[i - 1]) continue;
            if (candidates[i] > target) break;

            comb.push_back(candidates[i]);
            backtrack(candidates, target - candidates[i], i + 1, comb, result);
            comb.pop_back();
        }
    }
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        vector<vector<int>> result;
        vector<int> comb;

        backtrack(candidates, target, 0, comb, result);

        return result;
    }
};
