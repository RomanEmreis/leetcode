class Solution {
public:
    int trap(vector<int>& height) {
        if (height.size() == 2) return 0;

        int result = 0;
        int i = 0;
        int j = height.size() - 1;

        int left = height[i];
        int right = height[j];

        while (i < j) {
            if (left < right) {
                left = max(left, height[++i]);
                result += max((left - height[i]), 0);
            } else {
                right = max(right, height[--j]);
                result += max((right - height[j]), 0);
            }
        }

        return result;
    }
};
