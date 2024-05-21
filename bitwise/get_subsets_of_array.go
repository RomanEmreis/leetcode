/*
  Given an integer array nums of unique elements, return all possible subsets (the power set).
  The solution set must not contain duplicate subsets. Return the solution in any order.

  Example:
    Input: nums = [1,2,3]
    Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
*/
func subsets(nums []int) [][]int {
    n := len(nums);
    pn := 1 << n;
    result := make([][]int, 0);

    for i := 0; i < pn; i++ {
        subset := make([]int, 0);
        for j := 0; j < n; j++ {
            if i & (1 << j) > 0 {
                subset = append(subset, nums[j]);
            }
        }
        result = append(result, subset);
    }

    return result;
}
