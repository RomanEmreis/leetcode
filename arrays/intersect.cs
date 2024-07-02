/*
  Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

  Example:
    Input: nums1 = [1,2,2,1], nums2 = [2,2]
    Output: [2,2]
*/
public class Solution {
    public int[] Intersect(int[] nums1, int[] nums2) {
        if (nums1.Length > nums2.Length) return Intersect(nums2, nums1);

        Array.Sort(nums1);
        Array.Sort(nums2);

        List<int> result = [];

        int j = 0;
        for (int i = 0; i < nums1.Length; ++i) {
            int l = j, r = nums2.Length - 1;
            while (l <= r) {
                int m = l + (r - l) / 2;
                if (nums2[m] < nums1[i]) l = m + 1;
                else r = m - 1;
            }

            if (l != nums2.Length && nums2[l] == nums1[i]) {
                result.Add(nums1[i]);
                j = l + 1;
            }
        }

        return result.ToArray();
    }
}
