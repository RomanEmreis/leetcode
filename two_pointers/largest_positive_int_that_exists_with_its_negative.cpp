class Solution {
public:
    int findMaxK(vector<int>& nums) {
        if (nums.size() == 1) return -1;

        sort(nums.begin(), nums.end());

        int l = 0, r = nums.size() - 1, result = -1;
        while (l < r) {
            int sum = nums[l] + nums[r];
            if (sum == 0) {
                result = max(result, nums[r]);
                --r;
                ++l;
            } else if (sum > 0) {
                --r;
            } else {
                ++l;
            }
        }

        return result;
    }
};

static auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
