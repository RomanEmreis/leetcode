/*
  Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.
  Return the sorted array.

  Example:
    Input: nums = [1,1,2,2,2,3]
    Output: [3,1,1,2,2,2]
    Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
*/
func frequencySort(nums []int) []int {
    m := make(map[int]int);
    for _, n := range nums {
        if _, ok := m[n]; ok {
            m[n]++;
        } else {
            m[n] = 1;
        }
    }

    sort.Slice(nums, func(i, j int) bool {
        if m[nums[i]] == m[nums[j]] {
            return nums[i] > nums[j];
        } else {
            return m[nums[i]] < m[nums[j]];
        }
    });

    return nums;
}
