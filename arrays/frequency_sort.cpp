class Solution {
public:
    vector<int> sortArray(vector<int>& nums) {
        int mn = *min_element(nums.begin(), nums.end());
        int mx = *max_element(nums.begin(), nums.end());

        vector<int> freq(mx - mn + 1);
        for (const int& n : nums) ++freq[n - mn];

        int x = 0;
        for (int i = 0; i < freq.size(); ++i) {
            while (freq[i] > 0) {
                nums[x++] = i + mn;
                --freq[i];
            }
        }

        return nums;
    }
};
