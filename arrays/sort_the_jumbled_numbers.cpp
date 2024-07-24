/*
  You are given a 0-indexed integer array mapping which represents the mapping rule of a shuffled decimal system. mapping[i] = j means digit i should be mapped to digit j in this system.
  The mapped value of an integer is the new integer obtained by replacing each occurrence of digit i in the integer with mapping[i] for all 0 <= i <= 9.

  You are also given another integer array nums. Return the array nums sorted in non-decreasing order based on the mapped values of its elements.

  Notes:
    Elements with the same mapped values should appear in the same relative order as in the input.
    The elements of nums should only be sorted based on their mapped values and not be replaced by them.

  Example:
    Input: mapping = [8,9,4,0,2,1,3,5,7,6], nums = [991,338,38]
    Output: [338,38,991]
    Explanation: 
    Map the number 991 as follows:
      1. mapping[9] = 6, so all occurrences of the digit 9 will become 6.
      2. mapping[1] = 9, so all occurrences of the digit 1 will become 9.
    Therefore, the mapped value of 991 is 669.
    338 maps to 007, or 7 after removing the leading zeros.
    38 maps to 07, which is also 7 after removing leading zeros.
    Since 338 and 38 share the same mapped value, they should remain in the same relative order, so 338 comes before 38.
    Thus, the sorted array is [338,38,991].
*/
class Solution {
public:
    vector<int> sortJumbled(vector<int>& mapping, vector<int>& nums) {
        int n = nums.size();
        vector<pair<int, int>> m(n);

        for (int i = 0; i < nums.size(); ++i) {
            int j = 0;
            int p = 1;
            int v = nums[i];
            if (v == 0) {
                m[i] = {mapping[0], i};
                continue;
            }
            while (v != 0) {
                j += p * mapping[v % 10];
                p *= 10;
                v /= 10;
            }
            m[i] = {j, i};
        }

        sort(m.begin(), m.end());
        vector<int> result;
        for (const pair<int, int>& pair : m) {
            result.push_back(nums[pair.second]);
        }
        return result;
    }
};

static auto _ = [](){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return nullptr;
}();
