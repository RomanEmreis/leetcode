func sortArray(nums []int) []int {
    min, max := slices.Min(nums), slices.Max(nums);

    freq := make([]int, max - min + 1);

    for i := 0; i < len(nums); i++ {
        freq[nums[i] - min]++;
    }

    x := 0;
    for i := 0; i < len(freq); i++ {
        for freq[i] > 0 {
            nums[x] = i + min;
            freq[i]--;
            x++;
        }
    }

    return nums;
}
