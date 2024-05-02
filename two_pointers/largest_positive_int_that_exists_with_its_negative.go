func findMaxK(nums []int) int {
    if len(nums) == 1 {
        return -1;
    }

    sort.Ints(nums);

    l := 0;
    r := len(nums) - 1;
    result := -1;

    for l < r {
        sum := nums[l] + nums[r];
        if sum == 0 {
            result = int(math.Max(float64(result), float64(nums[r])));
            r--;
            l++;
        } else if sum > 0 {
            r--;
        } else {
            l++;
        }
    }

    return result;
}
