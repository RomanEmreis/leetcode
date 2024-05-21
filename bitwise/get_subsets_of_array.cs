/*
  Given an integer array nums of unique elements, return all possible subsets (the power set).
  The solution set must not contain duplicate subsets. Return the solution in any order.

  Example:
    Input: nums = [1,2,3]
    Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*/
public class Solution {
    public IList<IList<int>> Subsets(int[] nums) {
        IList<IList<int>> list = [];
        int pLen = 1 << nums.Length;

        for (int i = 0; i < pLen; ++i) {
            IList<int> subset = [];
            for (int j = 0; j < nums.Length; ++j) {
                if ((i & (1 << j)) > 0) subset.Add(nums[j]);
            }
            list.Add(subset);
        }
        return list;
    }
}
