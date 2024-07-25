public class Solution {
    public int[] SortArray(int[] nums) {
        int min = nums.Min();
        int max = nums.Max();

        Span<int> freq = stackalloc int[max - min + 1];

        for(int i = 0; i < nums.Length; ++i) ++freq[nums[i] - min];

        int x = 0;

        for(int i = 0; i < freq.Length; ++i) {
            while(freq[i] > 0) {
                nums[x++] = i + min;
                --freq[i];
            }
        }
        return nums;
    }
}
