/*
  Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

  Example:
    Input: nums1 = [1,2,2,1], nums2 = [2,2]
    Output: [2,2]
*/
class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        const int& n = nums1.size(), m = nums2.size();
        if (n > m) return intersect(nums2, nums1);

        sort(nums1.begin(), nums1.end());
        sort(nums2.begin(), nums2.end());

        vector<int> result;

        int j = 0;
        for (int i = 0; i < n; ++i) {
            int l = j, r = m - 1;
            while (l <= r) {
                int mid = l + (r - l) / 2;
                if (nums2[mid] < nums1[i]) l = mid + 1;
                else r = mid - 1;
            } 
            if (l < m && nums2[l] == nums1[i]) {
                result.push_back(nums1[i]);
                j = l + 1;
            }
        }

        return result;
    }
};
