public class Solution {
    public int Trap(int[] height) {
        if (height.Length == 2) {
            return 0;
        }

        int result = 0;
        
        int i = 0;
        int j = height.Length - 1;

        int left = height[i];
        int right = height[j];

        while (i < j) {
            if (left < right) {
                left = Math.Max(left, height[++i]);
                result += Math.Max((left - height[i]), 0);
            } else {
                right = Math.Max(right, height[--j]);
                result += Math.Max((right - height[j]), 0);
            }
        }

        return result;
    }
}
