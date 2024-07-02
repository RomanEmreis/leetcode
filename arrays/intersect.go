/*
  Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

  Example:
    Input: nums1 = [1,2,2,1], nums2 = [2,2]
    Output: [2,2]
*/
func intersect(nums1 []int, nums2 []int) []int {
    n, m := len(nums1), len(nums2);
    if n > m {
        return intersect(nums2, nums1);
    }

    sort.Ints(nums1);
    sort.Ints(nums2);

    result := make([]int, 0);

    j := 0;
    for i := 0; i < n; i++ {
        l, r := j, m - 1;
        for l <= r {
            mid := l + (r - l) / 2;
            if nums2[mid] < nums1[i] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        if l < m && nums2[l] == nums1[i] {
            result = append(result, nums1[i]);
            j = l + 1;
        }
    }

    return result;
}
